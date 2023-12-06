use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct ConvertionMap {
    name: String,
    source_ranges: Vec<(u64, u64)>,
    destination_ranges: Vec<(u64, u64)>,
    len: Vec<u64>,
    total_rules: u64,
}

impl ConvertionMap {
    fn new() -> ConvertionMap {
        ConvertionMap {
            name: String::new(),
            source_ranges: Vec::new(),
            destination_ranges: Vec::new(),
            len: Vec::new(),
            total_rules: 0,
        }
    }

    fn reset(&mut self) {
        self.name = String::new();
        self.source_ranges = Vec::new();
        self.destination_ranges = Vec::new();
        self.len = Vec::new();
        self.total_rules = 0;
    }

    fn handle_raw_line(&mut self, line: &str) {
        if line.find(":").is_some() {
            self.update_name_from_line(line);
        } else {
            self.add_map_rule_from_line(line);
        }
    }

    fn update_name_from_line(&mut self, line: &str) {
        let mut splited = line.split_whitespace();
        self.name = splited.next().unwrap().trim().to_string();
    }

    fn add_map_rule_from_line(&mut self, line: &str) {
        let mut values = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());

        if let (Some(destination), Some(source), Some(len)) =
            (values.next(), values.next(), values.next())
        {
            self.source_ranges.push((source, source + len - 1));
            self.destination_ranges
                .push((destination, destination + len - 1));
            self.len.push(len);
            self.total_rules += 1;
        } else {
            eprintln!("Error: not enough values");
        }
    }

    fn convert(&self, seed: u64) -> u64 {
        let mut new_seed = seed;
        for i in 0..self.total_rules {
            let i = i as usize;
            let sstart = self.source_ranges[i].0;
            let dstart = self.destination_ranges[i].0;
            let send = self.source_ranges[i].1;

            if new_seed >= sstart && new_seed <= send {
                new_seed = new_seed - sstart + dstart;
            }

            if new_seed != seed {
                break;
            }
        }
        new_seed
    }

    fn convert_range(&self, range: &(u64, u64)) -> Vec<(u64, u64)> {
        let mut changed: Vec<(u64, u64)> = Vec::new();
        let mut unchanged: Vec<(u64, u64)> = vec![*range];

        for source_range in &self.source_ranges {
            let mut new_ranges: Vec<(u64, u64)> = Vec::new();
            for current_range in &unchanged {
                if source_range.1 < current_range.0 || source_range.0 > current_range.1 {
                    // No intersection, add the current range to unchanged
                    new_ranges.push(*current_range);
                    continue;
                }

                let intersection = (
                    source_range.0.max(current_range.0),
                    source_range.1.min(current_range.1),
                );

                // Add the converted intersection to changed
                changed.push((self.convert(intersection.0), self.convert(intersection.1)));

                // Add any left-over range before the intersection
                if current_range.0 < intersection.0 {
                    new_ranges.push((current_range.0, intersection.0 - 1));
                }

                // Add any left-over range after the intersection
                if intersection.1 < current_range.1 {
                    new_ranges.push((intersection.1 + 1, current_range.1));
                }
            }
            unchanged = new_ranges;
        }

        // If no unchanged ranges and no changed ranges, add the original range
        if unchanged.is_empty() && changed.is_empty() {
            unchanged.push(*range);
        }

        unchanged.extend(changed);
        unchanged
    }
}

struct Seed {
    id: u64,
    range: (u64, u64),
}

fn parse_seeds(seeds_line: &str) -> Vec<Seed> {
    let splited = seeds_line.split(":");
    let seeds_str = splited.last().unwrap().split_whitespace();

    let mut seeds: Vec<Seed> = Vec::new();

    for (i, seed) in seeds_str.clone().enumerate() {
        if i % 2 == 0 {
            // For result2
            let range_start = seed.trim().parse::<u64>().unwrap();
            let range_length = seeds_str
                .clone()
                .nth(i + 1)
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap();

            seeds.push(Seed {
                id: range_start,
                range: (range_start, range_start + range_length - 1),
            });
        }
    }

    seeds
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day5 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut seeds: Vec<Seed> = Vec::new();

    // Load the maps
    let mut first_line = true;
    let mut maps: Vec<ConvertionMap> = Vec::new();
    let mut current_map: ConvertionMap = ConvertionMap::new();
    for line in reader.lines() {
        let line = line?;
        if first_line {
            seeds = parse_seeds(&line);
            first_line = false;
            continue;
        }
        if line.trim().is_empty() {
            maps.push(current_map.clone());
            current_map.reset();
        } else {
            current_map.handle_raw_line(&line);
        }
    }

    // Lasting one
    if maps.iter().last().unwrap().name != current_map.name {
        maps.push(current_map.clone());
    }

    let converted_unique_seeds: Vec<u64> = seeds
        .iter()
        .map(|seed| maps.iter().fold(seed.id, |acc, map| map.convert(acc)))
        .collect();

    let result1: u64 = *converted_unique_seeds.iter().min().unwrap_or(&0) as u64;
    println!("Result for part 1: {}", result1);

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for seed in &seeds {
        if seed.range.0 == 0 {
            continue;
        }

        let mut temp_ranges: Vec<(u64, u64)> = vec![seed.range];

        for map in &maps {
            if map.name.trim().is_empty() {
                continue;
            }
            let mut new_ranges: Vec<(u64, u64)> = Vec::new();
            while let Some(range) = temp_ranges.pop() {
                let converted_ranges = map.convert_range(&range);
                new_ranges.extend(converted_ranges);
            }
            temp_ranges.extend(new_ranges.clone());
        }

        ranges.extend(temp_ranges);
    }

    ranges.sort();

    let result2: u64 = ranges.iter().min_by_key(|&(start, _end)| start).unwrap().0;

    println!("Result for part 2: {}", result2);

    Ok(())
}
