use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Point {
    y: u64,
    x: u64,
}

impl Point {
    fn from(x: u64, y: u64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    id: u64,
    position: Point,
}

impl Galaxy {
    fn from(id: u64, position: Point) -> Self {
        Galaxy { id, position }
    }
}

struct Universe {
    galaxies: Vec<Galaxy>,
    galaxies_inflated: Vec<Galaxy>,
    rows_occupied: HashSet<u64>,
    cols_occupied: HashSet<u64>,
    pairs: Vec<(u64, u64)>,
    rows: u64,
    cols: u64,
}

impl Universe {
    fn new() -> Self {
        Universe {
            galaxies: Vec::new(),
            galaxies_inflated: Vec::new(),
            rows_occupied: HashSet::new(),
            cols_occupied: HashSet::new(),
            pairs: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }

    fn push_galaxy(&mut self, galaxy: Galaxy) {
        self.cols = self.cols.max(galaxy.position.x);
        self.rows = self.rows.max(galaxy.position.y);

        self.rows_occupied.insert(galaxy.position.y);
        self.cols_occupied.insert(galaxy.position.x);

        for stored_galaxy in &self.galaxies {
            self.pairs.push((galaxy.id, stored_galaxy.id));
        }

        self.galaxies.push(galaxy);
        self.galaxies_inflated.push(galaxy);
    }

    fn inflate_row(&mut self, row: u64, step: u64) {
        for (galaxy, original) in self.galaxies_inflated.iter_mut().zip(&self.galaxies) {
            if original.position.y > row {
                galaxy.position.y += step
            };
            self.rows = self.rows.max(galaxy.position.y);
        }
    }

    fn inflate_col(&mut self, col: u64, step: u64) {
        for (galaxy, original) in self.galaxies_inflated.iter_mut().zip(&self.galaxies) {
            if original.position.x > col {
                galaxy.position.x += step
            };
            self.cols = self.cols.max(galaxy.position.x);
        }
    }

    fn inflate(&mut self, step: u64) {
        if step <= 0 {
            println!("Warning: Could not handle step equal or below 0");
            return;
        }
        let step = step - 1;
        for i in 0..=self.rows {
            if !self.rows_occupied.contains(&i) {
                self.inflate_row(i, step);
            }
        }
        for j in 0..=self.cols {
            if !self.cols_occupied.contains(&j) {
                self.inflate_col(j, step);
            }
        }
    }

    fn process_sum_of_distances(&self) -> u64 {
        let mut id_map: HashMap<u64, &Point> = HashMap::new();
        for galaxy in &self.galaxies_inflated {
            id_map.insert(galaxy.id, &galaxy.position);
        }

        let mut total = 0;
        for pair in &self.pairs {
            let p1: &Point = id_map.get(&pair.0).unwrap();
            let p2: &Point = id_map.get(&pair.1).unwrap();
            let diff = (p1.x.max(p2.x) - p1.x.min(p2.x)) + (p1.y.max(p2.y) - p1.y.min(p2.y));
            total += diff;
        }
        total
    }

    fn deinflate(&mut self) {
        self.galaxies_inflated.clear();
        self.galaxies_inflated.extend(&self.galaxies);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day10 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut universe = Universe::new();

    let mut galaxy_count = 1;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                universe.push_galaxy(Galaxy::from(galaxy_count, Point::from(j as u64, i as u64)));
                galaxy_count += 1;
            }
        }
    }

    universe.inflate(2);
    let result1 = universe.process_sum_of_distances();
    println!("Result for part 1: {}", result1);

    universe.deinflate();
    universe.inflate(1000000);
    let result2 = universe.process_sum_of_distances();
    println!("Result for part 2: {}", result2);

    Ok(())
}
