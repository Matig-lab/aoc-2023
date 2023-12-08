use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Cycle;

#[derive(Copy, Clone)]
enum GraphDirection {
    None,
    Left,
    Right,
}

impl GraphDirection {
    fn from_char(dir_as_char: char) -> Self {
        match dir_as_char {
            'L' => GraphDirection::Left,
            'R' => GraphDirection::Right,
            _ => GraphDirection::None,
        }
    }
}

struct GraphDirectionsCycle {
    directions: Cycle<std::vec::IntoIter<GraphDirection>>,
}

impl GraphDirectionsCycle {
    fn new(instructions: Vec<GraphDirection>) -> Self {
        GraphDirectionsCycle {
            directions: instructions.into_iter().cycle(),
        }
    }

    fn next(&mut self) -> Option<GraphDirection> {
        self.directions.next()
    }
}

struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, entry: String, connections: Vec<String>) {
        self.adjacency_list
            .entry(entry.to_string())
            .or_insert_with(Vec::new)
            .extend(connections);
    }

    fn get_steps(&self, from: &str, to: Vec<&str>, directions: Vec<GraphDirection>) -> u32 {
        let mut steps = 0;
        let mut current: String = from.to_string();
        let mut instructions = GraphDirectionsCycle::new(directions);

        loop {
            let connections = self.adjacency_list.get(&current).unwrap();

            let next_step = match instructions.next() {
                Some(GraphDirection::Left) => connections.iter().nth(0).cloned().unwrap(),
                Some(GraphDirection::Right) => connections.iter().nth(1).cloned().unwrap(),
                _ => return 0,
            };

            steps += 1;

            let reached_end = to.iter().any(|end| next_step == **end);
            if reached_end {
                return steps;
            }
            current = next_step.clone();
        }
    }
}

fn parse_entry_from_line(line: &str) -> (String, Vec<String>) {
    let mut splited = line.split(" = ");

    let identifier = splited.next().unwrap().trim().to_owned();
    let connections = splited
        .next()
        .unwrap()
        .trim_matches(|c| c == '(' || c == ')')
        .split(", ")
        .map(|connection| connection.to_string())
        .collect();

    (identifier, connections)
}

fn lcm(numbers: &Vec<u32>) -> u64 {
    let highest = *numbers.iter().max().unwrap() as u64;
    let mut lcm = highest;

    loop {
        if numbers.iter().all(|n| lcm % *n as u64 == 0) {
            return lcm;
        }
        lcm += highest;
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day8 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut graph = Graph::new();
    let mut directions: Vec<GraphDirection> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        if i == 0 {
            for c in line.chars() {
                directions.push(GraphDirection::from_char(c));
            }
            continue;
        }

        let adjacency_list_entry = parse_entry_from_line(&line);
        graph.add_edge(adjacency_list_entry.0, adjacency_list_entry.1);
    }

    let result1 = graph.get_steps(&String::from("AAA"), vec!["ZZZ"], directions.clone());

    let ending_in_a: Vec<&str> = graph
        .adjacency_list
        .iter()
        .filter(|entry| entry.0.ends_with('A'))
        .map(|entry| entry.0.as_ref())
        .collect();

    let ending_in_z: Vec<&str> = graph
        .adjacency_list
        .iter()
        .filter(|entry| entry.0.ends_with('Z'))
        .map(|entry| entry.0.as_ref())
        .collect();

    let mut steps: Vec<u32> = Vec::new();
    for a in ending_in_a {
        steps.push(graph.get_steps(a, ending_in_z.clone(), directions.clone()));
    }

    let result2 = lcm(&steps);

    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
