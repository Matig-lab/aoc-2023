use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct PossibleNum {
    end: i32,
    start: i32,
    line: i32,
    num: i32,
}

struct SymPosition {
    idx: i32,
    line: i32,
}

impl PossibleNum {
    fn is_adjacent_to_sym(&self, sym: &SymPosition) -> bool {
        let mut is_adj = false;
        if sym.line == self.line - 1 || sym.line == self.line + 1 {
            is_adj = sym.idx >= self.start - 1 && sym.idx <= self.end + 1;
        }

        is_adj = if is_adj {
            is_adj
        } else {
            sym.line == self.line && (sym.idx == self.start - 1 || sym.idx == self.end + 1)
        };

        is_adj
    }
}

impl SymPosition {
    fn is_adjacent_to_possible(&self, possible: &PossibleNum) -> bool {
        let mut is_adj = false;
        for i in possible.start..=possible.end {
            if self.line == possible.line {
                if self.idx == i - 1 || self.idx == i + 1 {
                    is_adj = true;
                    break;
                }
            } else if self.line == possible.line - 1 || self.line == possible.line + 1 {
                if i - 1 == self.idx || i == self.idx || i + 1 == self.idx {
                    is_adj = true;
                    break;
                }
            }
        }
        is_adj
    }
}

struct State {
    possible_nums: Vec<PossibleNum>,
    valid_nums: Vec<PossibleNum>,
    symbols: Vec<SymPosition>,
    gear_symbols: Vec<SymPosition>,
}

impl State {
    fn new() -> State {
        State {
            possible_nums: Vec::new(),
            valid_nums: Vec::new(),
            symbols: Vec::new(),
            gear_symbols: Vec::new(),
        }
    }

    fn process_gears(self) -> i32 {
        let mut sum = 0;
        for sym in self.gear_symbols {
            let mut adjacents_nums: Vec<i32> = Vec::new();
            for num in &self.valid_nums {
                if sym.is_adjacent_to_possible(num) {
                    adjacents_nums.push(num.num);
                }
            }
            let mut product = 0;
            if adjacents_nums.len() == 2 {
                product = adjacents_nums.iter().fold(1, |acc, &x| acc * x);
            }
            sum += product;
        }
        sum
    }

    fn update(&mut self, current_line: i32) {
        // Delete old symbols and numsj
        self.symbols.retain(|sym| sym.line >= current_line - 1);
        self.possible_nums
            .retain(|num| num.line >= current_line - 1);

        // Delete numbs wich no adjacent symbol
        self.possible_nums.retain(|possible| {
            let adjacent = self
                .symbols
                .iter()
                .any(|sym| possible.is_adjacent_to_sym(&sym));

            // Add adjacent numbers to valid vector
            if adjacent {
                self.valid_nums.push(PossibleNum {
                    end: possible.end,
                    start: possible.start,
                    line: possible.line,
                    num: possible.num,
                });
            }

            !adjacent
        });
    }

    pub fn process_line(&mut self, line: &str, line_num: i32) {
        let mut processing_digit: bool = false;
        let mut possible_start: i32 = 0;
        let mut possible_str = String::new();
        let mut possible_start_setted = false;
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                processing_digit = true;
                if !possible_start_setted {
                    possible_start = i as i32;
                    possible_start_setted = true;
                }
                possible_str.push(c);
            } else {
                if !c.is_alphabetic() && c != '.' {
                    self.symbols.push(SymPosition {
                        idx: i as i32,
                        line: line_num,
                    });
                    if c == '*' {
                        self.gear_symbols.push(SymPosition {
                            idx: i as i32,
                            line: line_num,
                        });
                    }
                }
                if processing_digit {
                    processing_digit = false;
                    self.possible_nums.push(PossibleNum {
                        end: i as i32 - 1,
                        start: possible_start,
                        line: line_num,
                        num: possible_str.parse::<i32>().unwrap(),
                    });
                    possible_start = 0;
                    possible_str.truncate(0);
                    possible_start_setted = false;
                }
            }
        }

        // Add lasting number
        if processing_digit {
            self.possible_nums.push(PossibleNum {
                end: line.len() as i32 - 1,
                start: possible_start,
                line: line_num,
                num: possible_str.parse::<i32>().unwrap(),
            });
        }
        self.update(line_num);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day3 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut state = State::new();

    for (i, line) in reader.lines().enumerate() {
        state.process_line(&line?, i as i32);
    }

    let mut result1 = 0;
    for number in &state.valid_nums {
        result1 += number.num;
    }

    let result2 = state.process_gears();

    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
