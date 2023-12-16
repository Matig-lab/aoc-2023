use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Clone, Copy)]
enum PlatformDirection {
    North,
    West,
    South,
    East,
}

#[derive(Clone)]
struct Platform {
    grid: Vec<Vec<char>>,
}

impl Platform {
    fn new() -> Self {
        Platform { grid: Vec::new() }
    }

    fn add_line(&mut self, line: &str) {
        self.grid.push(line.chars().collect());
    }

    fn move_rock(&mut self, rock_pos: (usize, usize), direction: PlatformDirection) {
        let mut row = rock_pos.0;
        let mut col = rock_pos.1;

        match direction {
            PlatformDirection::North => {
                while row > 0 {
                    if self.grid[row - 1][col] == '.' {
                        self.grid[row][col] = '.';
                        self.grid[row - 1][col] = 'O';
                        row -= 1;
                    } else {
                        break;
                    }
                }
            }
            PlatformDirection::South => {
                while row < self.grid.len() - 1 {
                    if self.grid[row + 1][col] == '.' {
                        self.grid[row][col] = '.';
                        self.grid[row + 1][col] = 'O';
                        row += 1;
                    } else {
                        break;
                    }
                }
            }
            PlatformDirection::West => {
                while col > 0 {
                    if self.grid[row][col - 1] == '.' {
                        self.grid[row][col] = '.';
                        self.grid[row][col - 1] = 'O';
                        col -= 1;
                    } else {
                        break;
                    }
                }
            }
            PlatformDirection::East => {
                while col < self.grid[row].len() - 1 {
                    if self.grid[row][col + 1] == '.' {
                        self.grid[row][col] = '.';
                        self.grid[row][col + 1] = 'O';
                        col += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn do_cycle(&mut self) {
        self.pull_lever(PlatformDirection::North);
        self.pull_lever(PlatformDirection::West);
        self.pull_lever(PlatformDirection::South);
        self.pull_lever(PlatformDirection::East);
    }

    fn pull_lever(&mut self, direction: PlatformDirection) {
        let grid_clone = self.grid.clone();
        match direction.clone() {
            PlatformDirection::North | PlatformDirection::West => {
                for (i, line) in grid_clone.iter().enumerate() {
                    for (j, c) in line.iter().enumerate() {
                        if *c == 'O' {
                            self.move_rock((i, j), direction);
                        }
                    }
                }
            }
            PlatformDirection::South | PlatformDirection::East => {
                for i in (0..grid_clone.len()).rev() {
                    let line = &self.grid[i];
                    for j in (0..line.len()).rev() {
                        let c = self.grid[i][j];
                        if c == 'O' {
                            self.move_rock((i, j), direction);
                        }
                    }
                }
            }
        }
    }

    fn calc_north_load(&self) -> i32 {
        let grid_clone = self.grid.clone();

        let mut load: i32 = 0;
        let grid_height = self.grid.len() as i32;
        for (i, line) in grid_clone.iter().enumerate() {
            for c in line.iter() {
                if *c == 'O' {
                    load += grid_height - i as i32;
                }
            }
        }
        load
    }

    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|inner_vec| inner_vec.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day14 <filename>");
        std::process::exit(1);
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut p1 = Platform::new();

    for line in reader.lines() {
        p1.add_line(&line?);
    }

    let mut p2 = p1.clone();
    p1.pull_lever(PlatformDirection::North);

    println!("Result for part 1: {}", p1.calc_north_load());

    let mut hash_table: HashMap<String, u32> = HashMap::new();

    const CYCLES: u32 = 1000000000;
    let mut start_of_loop_str = String::new();
    let mut i = 0;
    while i < CYCLES {
        p2.do_cycle();
        let current_grid_str = p2.to_string();
        if current_grid_str == start_of_loop_str {
            break;
        }
        if start_of_loop_str.is_empty() && hash_table.contains_key(&current_grid_str) {
            start_of_loop_str = current_grid_str.clone();
        }

        hash_table.entry(current_grid_str.clone()).or_insert(i);
        i += 1;
    }


    let steps_to_loop = hash_table.get(&start_of_loop_str).unwrap();
    let loop_len = hash_table.len() as u32 - steps_to_loop;
    let place_in_loop = CYCLES % loop_len;

    for _ in 0..place_in_loop + loop_len - i % loop_len - 1 {
        p2.do_cycle();
    }

    println!("Result for part 2: {}", p2.calc_north_load());

    Ok(())
}
