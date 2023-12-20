use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Part {
    X,
    M,
    A,
    S,
}

impl Part {
    fn from_char(part: char) -> Self {
        match part {
            'x' => return Part::X,
            'm' => return Part::M,
            'a' => return Part::A,
            's' => return Part::S,
            _ => panic!("Invalid part char"),
        }
    }

    fn as_idx(&self) -> usize {
        match self {
            Part::X => return 0,
            Part::M => return 1,
            Part::A => return 2,
            Part::S => return 3,
        }
    }
}

const TOTAL_PARTS: usize = 4;
const START_LABEL: &str = "in";
const ACCEPTED_LABEL: &str = "A";
const REJECTED_LABEL: &str = "R";
const NEXT_INSTRUCTION_LABEL: &str = "NEXT";

enum Operation {
    None,
    GreaterThan,
    LessThan,
}

struct Rule {
    part_idx: usize,
    operation: Operation,
    comparator: u32,
    termination: String,
}

impl Rule {
    fn from(rule_str: &str) -> Self {
        let mut op = Operation::None;
        let mut part = Part::A.as_idx();
        let termination: String;
        let mut cmp = 0;

        if rule_str.contains(":") {
            let mut splited = rule_str.split(":");
            let cmp_part = splited.next().unwrap().chars().collect::<String>();
            termination = splited.next().unwrap().to_string();
            part = Part::from_char(cmp_part.chars().next().unwrap()).as_idx();
            op = match cmp_part.chars().nth(1).unwrap() {
                '>' => Operation::GreaterThan,
                '<' => Operation::LessThan,
                _ => Operation::None,
            };
            cmp = cmp_part
                .chars()
                .skip(2)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
        } else {
            termination = rule_str.to_string();
        }

        Rule {
            part_idx: part,
            operation: op,
            comparator: cmp,
            termination,
        }
    }
}

impl Rule {
    fn process_part(&self, part: &[u32; TOTAL_PARTS]) -> &str {
        match self.operation {
            Operation::GreaterThan => {
                if part[self.part_idx] > self.comparator {
                    return &self.termination;
                } else {
                    return NEXT_INSTRUCTION_LABEL;
                }
            }
            Operation::LessThan => {
                if part[self.part_idx] < self.comparator {
                    return &self.termination;
                } else {
                    return NEXT_INSTRUCTION_LABEL;
                }
            }
            Operation::None => return &self.termination,
        }
    }

    fn process_range(
        &self,
        range: [(u32, u32); TOTAL_PARTS],
        inverted: bool,
    ) -> [(u32, u32); TOTAL_PARTS] {
        let mut offset = 1;
        let mut op = &self.operation;

        if inverted {
            offset = 0;
            op = match op {
                Operation::GreaterThan => &Operation::LessThan,
                Operation::LessThan => &Operation::GreaterThan,
                Operation::None => op,
            }
        }
        let mut processed_range = range.clone();
        match op {
            Operation::GreaterThan => {
                processed_range[self.part_idx].0 =
                    range[self.part_idx].0.max(self.comparator + offset);
            }
            Operation::LessThan => {
                processed_range[self.part_idx].1 =
                    range[self.part_idx].1.min(self.comparator - offset);
            }
            Operation::None => (),
        }
        processed_range
    }
}

struct Machine {
    workflow_table: HashMap<String, Vec<Rule>>,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            workflow_table: HashMap::new(),
        }
    }

    fn add_workflow(&mut self, label: &str, instructions: Vec<Rule>) {
        self.workflow_table.insert(label.to_string(), instructions);
    }

    fn combinations_accepted(&self, mut ranges: [(u32, u32); TOTAL_PARTS], workflow: &str) -> u64 {
        let mut accepted_combinations: u64 = 0;

        if let Some(wf) = self.workflow_table.get(workflow) {
            for rule in wf {
                let accepted_ranges = rule.process_range(ranges.clone(), false);
                ranges = rule.process_range(ranges, true);
                let next: &str = &rule.termination;
                match next {
                    ACCEPTED_LABEL => {
                        let combinations = accepted_ranges
                            .iter()
                            .map(|&(start, end)| if start < end {end - start + 1} else {0} as u64)
                            .product::<u64>();
                        // println!("{:?} - {}", accepted_ranges, combinations);
                        accepted_combinations += combinations;
                    }
                    REJECTED_LABEL => (),
                    _ => accepted_combinations += self.combinations_accepted(accepted_ranges, next),
                }
            }
        }
        accepted_combinations
    }

    fn approve_parts(&self, parts: &[u32; TOTAL_PARTS], workflow: &str) -> bool {
        if let Some(wf) = self.workflow_table.get(workflow) {
            for rule in wf {
                match rule.process_part(parts) {
                    ACCEPTED_LABEL => return true,
                    REJECTED_LABEL => return false,
                    NEXT_INSTRUCTION_LABEL => continue,
                    _ => return self.approve_parts(parts, &rule.termination),
                }
            }
        }
        false
    }
}

fn parse_workflow(line: &str) -> (String, Vec<Rule>) {
    let mut parts = line.split('{');
    let label = parts.next().unwrap().trim().to_string();
    let rules_str = parts.next().unwrap().trim_matches('}');

    let rules: Vec<Rule> = rules_str
        .split(',')
        .map(|rule_str| Rule::from(rule_str.trim()))
        .collect();

    (label, rules)
}

fn parse_xmas(line: &str) -> [u32; 4] {
    let mut xmas_values = HashMap::new();
    let mut line_cleaned = line.replace("{", "");
    line_cleaned = line_cleaned.replace("}", "");

    let parts: Vec<&str> = line_cleaned.split(',').collect();

    for part in parts {
        let key_value: Vec<&str> = part.split('=').collect();
        if key_value.len() == 2 {
            let key = key_value[0].trim();
            let value: u32 = key_value[1].trim().parse().unwrap_or(0);

            xmas_values.insert(key, value);
        }
    }

    let xmas_array = [
        xmas_values.get("x").unwrap_or(&0).clone(),
        xmas_values.get("m").unwrap_or(&0).clone(),
        xmas_values.get("a").unwrap_or(&0).clone(),
        xmas_values.get("s").unwrap_or(&0).clone(),
    ];

    xmas_array
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day19 <filename>");
        std::process::exit(1);
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut switch = false;
    let mut xmas_parts: Vec<[u32; TOTAL_PARTS]> = Vec::new();
    let mut machine: Machine = Machine::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            switch = true;
            continue;
        }
        if switch {
            xmas_parts.push(parse_xmas(&line));
        } else {
            let (label, instructions) = parse_workflow(&line);
            machine.add_workflow(&label, instructions);
        }
    }

    let mut result1 = 0;
    for parts in xmas_parts {
        result1 += if machine.approve_parts(&parts, START_LABEL) {
            parts.iter().sum()
        } else {
            0
        };
    }
    println!("Result for part 1: {}", result1);

    let ranges: [(u32, u32); TOTAL_PARTS] = [(1, 4000); TOTAL_PARTS];
    let result2 = machine.combinations_accepted(ranges, START_LABEL);
    println!("Result for part 2: {}", result2);

    Ok(())
}
