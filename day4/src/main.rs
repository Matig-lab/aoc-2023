use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Card {
    id: u32,
    winning: Vec<u32>,
    actual: Vec<u32>,
    points: u32,
    matches: u32,
    instances: u32,
}

impl Card {
    fn new() -> Card {
        Card {
            id: 0,
            winning: Vec::new(),
            actual: Vec::new(),
            points: 0,
            matches: 0,
            instances: 1,
        }
    }

    fn process_id(&mut self, id_part: &str) {
        self.id = id_part
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or_default();
    }

    fn process_winning_part(&mut self, winning_part: &str) {
        self.winning.extend(
            winning_part
                .split_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap_or_default()),
        );
    }

    fn process_actual_num_part(&mut self, actual_num_part: &str) {
        self.actual.extend(
            actual_num_part
                .split_whitespace()
                .map(|num| num.trim().parse::<u32>().unwrap_or_default()),
        )
    }

    fn new_from_line(line: &str) -> Card {
        let mut line_parts = line.split("|");
        let left_part = line_parts.next().unwrap_or_default().trim();
        let actual_num_part = line_parts.next().unwrap_or_default().trim();

        let mut left_splited = left_part.split(":");
        let id_part = left_splited.next().unwrap_or_default().trim();
        let winning_part = left_splited.next().unwrap_or_default().trim();

        let mut card = Card::new();
        card.process_id(id_part);
        card.process_winning_part(winning_part);
        card.process_actual_num_part(actual_num_part);

        card.calc_points();
        card
    }

    fn calc_points(&mut self) {
        self.matches = self
            .actual
            .iter()
            .filter(|&num| self.winning.contains(num))
            .count() as u32;

        if self.matches >= 1 {
            self.points = 1;
        }

        for _i in 1..self.matches {
            self.points *= 2;
        }
    }
}

const TOTAL_CARDS: usize = 200;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day4 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    // Lookup table
    let mut copies_pool: [u32; TOTAL_CARDS] = [0; TOTAL_CARDS];

    let mut result1 = 0;
    for (_i, line) in reader.lines().enumerate() {
        let mut card = Card::new_from_line(&line?);
        card.instances += copies_pool[card.id as usize];
        copies_pool[card.id as usize] += 1;
        for copy_won in 0..card.matches {
            // The problem ensures that the index is within limits
            copies_pool[(card.id + copy_won + 1) as usize] += card.instances;
        }
        result1 += card.points;
    }

    let result2: u32 = copies_pool.iter().sum();
    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
