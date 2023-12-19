use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time::Duration};


#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum CrucibleDirection {
    None,
    North,
    South,
    West,
    East,
}

#[derive(Clone, Eq, PartialEq)]
struct Crucible {
    pos: (i32, i32),
    direction: CrucibleDirection,
    last_direction: CrucibleDirection,
    visited: Vec<(i32, i32)>,
    straight_steps: u8,
    current_heat_loss: u32,
    cost: u32,
    ultra: bool,
    max_steps: u8,
    min_steps: u8,
}

impl Crucible {
    fn new(is_ultra: bool) -> Self {
        Crucible {
            pos: (0, 0),
            direction: CrucibleDirection::East,
            last_direction: CrucibleDirection::None,
            visited: Vec::new(),
            straight_steps: 0,
            current_heat_loss: 0,
            ultra: is_ultra,
            max_steps: if is_ultra { 10 } else { 3 },
            min_steps: if is_ultra { 4 } else { 1 },
            cost: 0,
        }
    }

    fn possible_directions(&self) -> Vec<CrucibleDirection> {
        let mut directions: Vec<CrucibleDirection> = Vec::new();
        match self.direction {
            CrucibleDirection::North | CrucibleDirection::South => {
                directions.push(self.direction.clone());
                directions.push(CrucibleDirection::East);
                directions.push(CrucibleDirection::West);
            }
            CrucibleDirection::West | CrucibleDirection::East => {
                directions.push(self.direction.clone());
                directions.push(CrucibleDirection::South);
                directions.push(CrucibleDirection::North);
            }
            _ => (),
        }
        directions
    }

    fn change_direction(&mut self, new_direction: CrucibleDirection) {
        if self.straight_steps >= self.min_steps {
            self.direction = new_direction;
        }
    }

    fn step(&mut self) -> bool {
        if self.last_direction == self.direction {
            self.straight_steps += 1;
        } else {
            self.straight_steps = 1;
        }

        if self.straight_steps > self.max_steps {
            return false;
        }

        let mut new_pos = self.pos;
        match self.direction {
            CrucibleDirection::North => new_pos.0 -= 1,
            CrucibleDirection::South => new_pos.0 += 1,
            CrucibleDirection::West => new_pos.1 -= 1,
            CrucibleDirection::East => new_pos.1 += 1,
            _ => (),
        }

        if self.visited.contains(&new_pos) {
            return false;
        }

        self.visited.push(new_pos);
        self.pos = new_pos;
        self.last_direction = self.direction;
        true
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct CityBlock {
    map: Vec<Vec<u32>>,
    min_heat_loss: u32,
    avg_tile: u32,
    best_crucible: Crucible,
}

impl CityBlock {
    fn new() -> Self {
        CityBlock {
            map: Vec::new(),
            min_heat_loss: i32::MAX as u32,
            avg_tile: 0,
            best_crucible: Crucible::new(false),
        }
    }

    fn add_line(&mut self, line: &str) {
        self.map
            .push(line.chars().map(|c| (c as u8 - '0' as u8) as u32).collect());
    }

    fn reset(&mut self) {
        self.min_heat_loss = i32::MAX as u32;
        self.best_crucible = Crucible::new(false);
        self.setup();
    }

    fn setup(&mut self) {
        let sum: u32 = self.map.iter().map(|v| v.iter().sum::<u32>()).sum();

        self.avg_tile =
            (sum as f32 / (self.map.len() * self.map[0].len()) as f32).floor() as u32 - 1;
        self.min_heat_loss =
            self.avg_tile * ((self.map.len() * self.map[0].len()) as f32 / 4.0).round() as u32;
    }

    fn is_within_bounds(&self, pos: (i32, i32)) -> bool {
        if pos.0 < 0 || pos.0 >= self.map.len() as i32 {
            return false;
        }
        if pos.1 < 0 || pos.1 >= self.map[0].len() as i32 {
            return false;
        }
        true
    }

    fn calc_cost(&self, crucible: &mut Crucible, destination: (i32, i32)) {
        let distance: u32 = (destination.0 - crucible.pos.0).abs() as u32
            + (destination.1 - crucible.pos.1).abs() as u32;
        let estimated_heat_loss = distance * self.avg_tile;
        crucible.cost = estimated_heat_loss + crucible.current_heat_loss;
    }

    fn minimize_heat_loss(
        &mut self,
        crucible_queue: &mut BinaryHeap<Crucible>,
        destination: (i32, i32),
    ) {
        let mut cache: HashMap<((i32, i32), CrucibleDirection, u8), u32> = HashMap::new();
        while let Some(mut crucible) = crucible_queue.pop() {
            let initial_state = crucible.clone();
            let new_directions = crucible.possible_directions();

            for dir in new_directions {
                crucible = initial_state.clone();
                crucible.change_direction(dir);
                if crucible.step() && self.is_within_bounds(crucible.pos) {
                    crucible.current_heat_loss +=
                        self.map[crucible.pos.0 as usize][crucible.pos.1 as usize];
                    self.calc_cost(&mut crucible, destination);

                    if let Some(cached) =
                        cache.get_mut(&(crucible.pos, crucible.direction, crucible.straight_steps))
                    {
                        if cached > &mut crucible.cost {
                            *cached = crucible.cost;
                        } else {
                            continue;
                        }
                    } else {
                        cache.insert(
                            (crucible.pos, crucible.direction, crucible.straight_steps),
                            crucible.cost,
                        );
                    }

                    if crucible.pos == destination {
                        if crucible.straight_steps < crucible.min_steps {
                            continue;
                        }
                        let last = self.min_heat_loss;
                        self.min_heat_loss = crucible.current_heat_loss.min(self.min_heat_loss);
                        if last != self.min_heat_loss {
                            self.best_crucible = crucible.clone();
                        }
                        continue;
                    }

                    crucible_queue.push(crucible);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn visualize_all_paths(
    city_block: &CityBlock,
    current_crucible: &Crucible,
    crucible_queue: &BinaryHeap<Crucible>,
    lowest_heat_loss: u32,
    destination: (i32, i32),
    overwrite: bool,
) {
    let queued_paths: Vec<Crucible> = crucible_queue.iter().cloned().collect();
    for (i, row) in city_block.map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let pos = (i as i32, j as i32);

            if pos == current_crucible.pos {
                print!("\x1B[1;33mC\x1B[0m");
            } else if current_crucible.visited.contains(&pos) {
                print!("\x1B[1;33m{}\x1B[0m", cell);
            } else if queued_paths.iter().any(|c| c.visited.contains(&pos)) {
                print!("\x1B[1;30m{}\x1B[0m", cell);
            } else if pos == destination {
                print!("\x1B[1;31mX\x1B[0m");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }

    println!(
        "Lowest Heat Loss: {}                     ",
        lowest_heat_loss
    );
    if overwrite {
        print!("\x1B[{}A", city_block.map.len() + 1);
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
}

#[allow(dead_code)]
fn visualize_path(
    city_block: &CityBlock,
    crucible: &Crucible,
    lowest_heat_loss: u32,
    destination: (i32, i32),
    overwrite: bool,
) {
    for (i, row) in city_block.map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if (i as i32, j as i32) == crucible.pos {
                print!("\x1B[1;33mC\x1B[0m");
            } else if crucible.visited.contains(&(i as i32, j as i32)) {
                print!("\x1B[1;33m{}\x1B[0m", cell);
            } else if (i as i32, j as i32) == destination {
                print!("\x1B[1;31mX\x1B[0m");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }

    println!(
        "Lowest Heat Loss: {}                     ",
        lowest_heat_loss
    );
    if overwrite {
        print!("\x1B[{}A", city_block.map.len() + 1);
    }

    thread::sleep(Duration::from_millis(500));
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day17 <filename>");
        std::process::exit(1);
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut city_block = CityBlock::new();

    for line in reader.lines() {
        let line_str = line?;
        city_block.add_line(&line_str);
    }

    city_block.setup();

    let destination = (
        city_block.map.len() as i32 - 1,
        city_block.map[0].len() as i32 - 1,
    );
    let mut crucible_queue: BinaryHeap<Crucible> = BinaryHeap::new();
    crucible_queue.push(Crucible::new(false));

    city_block.minimize_heat_loss(&mut crucible_queue, destination);
    println!("Result for part 1: {}", city_block.min_heat_loss);

    crucible_queue.clear();
    crucible_queue.push(Crucible::new(true));
    city_block.reset();
    city_block.minimize_heat_loss(&mut crucible_queue, destination);

    println!("Result for part 2: {}", city_block.min_heat_loss);

    // visualize_all_paths(
    //     &city_block,
    //     &city_block.best_crucible,
    //     &crucible_queue,
    //     city_block.min_heat_loss,
    //     destination,
    //     false,
    // );

    Ok(())
}
