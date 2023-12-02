use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new() -> Set {
        Set {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn add_element(&mut self, element: &str) {
        let mut e_parts = element.split_whitespace();
        let e_val = e_parts
            .next()
            .unwrap_or_default()
            .trim()
            .parse::<u32>()
            .unwrap_or_default();
        let e_id = e_parts.next().unwrap_or_default().trim();

        match e_id {
            "green" => self.green = e_val,
            "red" => self.red = e_val,
            "blue" => self.blue = e_val,
            _ => (),
        }
    }
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn new() -> Game {
        Game {
            id: 0,
            sets: Vec::new(),
        }
    }

    fn new_from_str(line: &str) -> Game {
        let mut new_game = Game::new();
        let mut str_parts = line.split(":");
        let game_str = str_parts.next().unwrap_or_default().trim();
        let sets_str = str_parts.next().unwrap_or_default().trim();

        // Get game ID
        if let Some(id_str) = game_str.split_whitespace().nth(1) {
            if let Ok(id) = id_str.parse::<u32>() {
                new_game.id = id;
            }
        }

        // Load sets
        new_game.sets = sets_str
            .split(";")
            .map(|set| {
                let set_elements = set.split(",").collect::<Vec<&str>>();
                let mut set = Set::new();

                for element in set_elements {
                    set.add_element(element);
                }

                set
            })
            .collect();

        new_game
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day2 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    // part 1
    let max_reds = 12;
    let max_greens = 13;
    let max_blues = 14;
    let mut result1 = 0;
    let mut result2 = 0;

    for line in reader.lines() {
        let temp_game = Game::new_from_str(&line.unwrap_or_default());
        let mut ok: bool = true;
        let mut min_of_blues = 0;
        let mut min_of_reds = 0;
        let mut min_of_greens = 0;

        for set in temp_game.sets {
            // part 1
            if ok && (set.red > max_reds || set.green > max_greens || set.blue > max_blues) {
                ok = false;
            }

            // part 2
            min_of_blues = if set.blue > min_of_blues {
                set.blue
            } else {
                min_of_blues
            };
            min_of_reds = if set.red > min_of_reds {
                set.red
            } else {
                min_of_reds
            };
            min_of_greens = if set.green > min_of_greens {
                set.green
            } else {
                min_of_greens
            };
        }

        // part 1
        if ok {
            result1 += temp_game.id;
        }

        // part 2
        let pow_of_set = min_of_blues * min_of_reds * min_of_greens;
        result2 += pow_of_set;
    }

    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
