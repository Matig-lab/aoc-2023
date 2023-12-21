use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Debug, Clone)]
enum ModuleType {
    None,
    FlipFlop,
    Conjunction,
    Broadcast,
}

const BROADCASTER_LABEL: &str = "broadcaster";
const BUTTON_LABEL: &str = "button";
const RX_MODULE_LABEL: &str = "rx";

#[derive(Eq, PartialEq, Clone, Debug)]
enum PulseType {
    None,
    LowPulse,
    HighPulse,
}

impl PulseType {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        match *self {
            PulseType::LowPulse => String::from("-low"),
            PulseType::HighPulse => String::from("-high"),
            PulseType::None => String::from("-none"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum ModuleState {
    On,
    Off,
}

impl ModuleState {
    fn inverse(&self) -> Self {
        if *self == ModuleState::On {
            return ModuleState::Off;
        }
        ModuleState::On
    }
}

#[derive(Debug, Clone)]
struct Module {
    label: String,
    module_type: ModuleType,
    destinations: Vec<String>,
    last_recived: HashMap<String, PulseType>,
    state: ModuleState,
}

impl Module {
    fn new_of_type(type_string: &str, label: String) -> Self {
        let module_type = match type_string {
            "&" => ModuleType::Conjunction,
            "%" => ModuleType::FlipFlop,
            BROADCASTER_LABEL => ModuleType::Broadcast,
            _ => ModuleType::None,
        };
        Module {
            label,
            module_type,
            destinations: Vec::new(),
            last_recived: HashMap::new(),
            state: ModuleState::Off,
        }
    }

    fn add_destination(&mut self, other_label: String) {
        self.destinations.push(other_label);
    }

    fn save_connection(&mut self, other_label: String) {
        if self.module_type == ModuleType::Conjunction {
            self.last_recived.insert(other_label, PulseType::LowPulse);
        }
    }

    fn process_response(&mut self, pulse: &PulseType, from: &str) -> PulseType {
        match self.module_type {
            ModuleType::FlipFlop => {
                if *pulse == PulseType::LowPulse {
                    self.state = self.state.inverse();
                    if self.state == ModuleState::On {
                        return PulseType::HighPulse;
                    }
                    return PulseType::LowPulse;
                }
                return PulseType::None;
            }
            ModuleType::Conjunction => {
                if let Some(last) = self.last_recived.get_mut(from) {
                    *last = pulse.clone();
                }
                if self
                    .last_recived
                    .values()
                    .all(|p| *p == PulseType::HighPulse)
                {
                    return PulseType::LowPulse;
                }
                return PulseType::HighPulse;
            }
            ModuleType::Broadcast => {
                return pulse.clone();
            }
            _ => return PulseType::None,
        }
    }

    fn recive_and_process(&mut self, pulse: &PulseType, from: &str) -> (PulseType, Vec<String>) {
        let res = self.process_response(pulse, from);
        (res, self.destinations.clone())
    }
}

#[derive(Clone)]
struct Graph {
    adj_matrix: HashMap<String, Module>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj_matrix: HashMap::new(),
        }
    }

    fn press_button(&mut self) -> (u32, u32) {
        let mut q: VecDeque<(PulseType, String, String)> = VecDeque::new();
        let mut low_count = 0u32;
        let mut high_count = 0u32;
        q.push_back((
            PulseType::LowPulse,
            BROADCASTER_LABEL.to_string(), // To
            BUTTON_LABEL.to_string(),      // From
        ));

        while let Some(send_info) = q.pop_front() {
            match send_info.0 {
                PulseType::None => (),
                PulseType::LowPulse => low_count += 1,
                PulseType::HighPulse => high_count += 1,
            }
            if let Some(module) = self.adj_matrix.get_mut(&send_info.1) {
                let new_send_info = module.recive_and_process(&send_info.0, &send_info.2);

                if new_send_info.0 == PulseType::None {
                    continue;
                }
                for receiver in new_send_info.1 {
                    q.push_back((
                        new_send_info.0.clone(),
                        receiver.to_string(),
                        module.label.clone(),
                    ));
                }
            }
        }
        (low_count, high_count)
    }

    fn press_button_and_track(&mut self, target_module: &str, it: u32, mod_map: &mut HashMap<String, u32>) {
        let mut q: VecDeque<(PulseType, String, String)> = VecDeque::new();
        q.push_back((
            PulseType::LowPulse,
            BROADCASTER_LABEL.to_string(), // To
            BUTTON_LABEL.to_string(),      // From
        ));

        while let Some(send_info) = q.pop_front() {
            if let Some(module) = self.adj_matrix.get_mut(&send_info.1) {
                let new_send_info = module.recive_and_process(&send_info.0, &send_info.2);

                if new_send_info.0 == PulseType::None {
                    continue;
                }
                for receiver in new_send_info.1 {
                    if receiver == target_module && !mod_map.contains_key(&receiver) && new_send_info.0 == PulseType::HighPulse {
                        mod_map.insert(module.label.clone(), it);
                    }

                    q.push_back((
                        new_send_info.0.clone(),
                        receiver.to_string(),
                        module.label.clone(),
                    ));
                }
            }
        }
    }

    fn cycles_to_rx(&mut self) -> u64 {
        let mut module_map: HashMap<String, u32> = HashMap::new();

        let mut module_prev_to_rx = String::new();
        for module in self.adj_matrix.values() {
            for dest in &module.destinations {
                if *dest == RX_MODULE_LABEL {
                    module_prev_to_rx = module.label.clone();
                    break;
                }
            }
        }

        let mut modules_to_prev_to_rx = 0;
        for module in self.adj_matrix.values() {
            for dest in &module.destinations {
                if *dest == module_prev_to_rx {
                    modules_to_prev_to_rx += 1;
                    break;
                }
            }
        }

        let mut cycles = 0u32;
        loop {
            cycles += 1;
            self.press_button_and_track(&module_prev_to_rx, cycles, &mut module_map);

            if module_map.len() == modules_to_prev_to_rx {
                break;
            }

        }
        lcm(module_map.values().cloned().map(|v| v as u64).collect())
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(numbers: Vec<u64>) -> u64 {
    numbers.into_iter().fold(1, |acc, x| acc * x / gcd(acc, x))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day20 <filename>");
        std::process::exit(1);
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut graph = Graph::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(" -> ").collect();

        if parts.len() == 2 {
            let mut source_label = parts[0].trim().to_string();
            let connections: Vec<String> =
                parts[1].trim().split(", ").map(|s| s.to_string()).collect();

            let mut sym: &str = BROADCASTER_LABEL;
            match source_label.clone().chars().nth(0).unwrap() {
                '&' => {
                    sym = "&";
                    source_label = source_label.replace("&", "");
                }
                '%' => {
                    sym = "%";
                    source_label = source_label.replace("%", "");
                }
                _ => (),
            }

            let source_module = graph
                .adj_matrix
                .entry(source_label.to_string())
                .or_insert_with(|| Module::new_of_type(sym, source_label.to_string()));

            for conn in connections {
                source_module.add_destination(conn);
            }
        }
    }

    let temp_matrix = graph.adj_matrix.clone();
    for module in graph.adj_matrix.values_mut() {
        if module.module_type == ModuleType::Conjunction {
            for sub_module in temp_matrix.values() {
                if sub_module.destinations.contains(&module.label) {
                    module.save_connection(sub_module.label.clone());
                }
            }
        }
    }

    let mut graph_bak = graph.clone();
    let mut low_count = 0u32;
    let mut high_count = 0u32;
    for _ in 0..1000 {
        let sended_pulses = graph.press_button();
        low_count += sended_pulses.0;
        high_count += sended_pulses.1;
    }

    let result1 = high_count * low_count;

    println!("Result for part 1: {}", result1);

    let result2 = graph_bak.cycles_to_rx();
    println!("Result for part 2: {}", result2);

    Ok(())
}
