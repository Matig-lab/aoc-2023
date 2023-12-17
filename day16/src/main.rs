use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TOTAL_DIRECTIONS: usize = 4;
#[derive(Eq, PartialEq, Clone, Copy)]
enum BeamDirection {
    North,
    South,
    West,
    East,
}

impl BeamDirection {
    fn as_idx(&self) -> usize {
        match self {
            BeamDirection::North => 0,
            BeamDirection::South => 1,
            BeamDirection::West => 2,
            BeamDirection::East => 3,
        }
    }
}

#[derive(Copy, Clone)]
struct Beam {
    position: (i32, i32),
    direction: BeamDirection,
}

impl Beam {
    fn new() -> Self {
        Beam {
            position: (0, 0),
            direction: BeamDirection::East,
        }
    }

    fn from(position: (i32, i32), direction: BeamDirection) -> Self {
        Beam {
            position,
            direction,
        }
    }

    fn change_direction(&mut self, direction: BeamDirection) {
        self.direction = direction;
    }

    fn step(&mut self) {
        match self.direction {
            BeamDirection::North => self.position.0 -= 1,
            BeamDirection::South => self.position.0 += 1,
            BeamDirection::West => self.position.1 -= 1,
            BeamDirection::East => self.position.1 += 1,
        }
    }
}

#[derive(Clone)]
struct EnergizedTile {
    from_direction: [bool; TOTAL_DIRECTIONS],
}

impl EnergizedTile {
    fn new() -> Self {
        EnergizedTile {
            from_direction: [false, false, false, false],
        }
    }
}

#[derive(Clone)]
struct Contraption {
    grid: Vec<Vec<char>>,
    energized: Vec<Vec<EnergizedTile>>,
}

impl Contraption {
    fn new() -> Self {
        Contraption {
            grid: Vec::new(),
            energized: Vec::new(),
        }
    }

    fn add_line(&mut self, line: &str) {
        self.grid.push(line.chars().collect());
        self.energized.push(Vec::new());
        for _ in 0..line.len() {
            self.energized
                .last_mut()
                .unwrap()
                .push(EnergizedTile::new());
        }
    }

    fn reset(&mut self) {
        for row in &mut self.energized {
            for tile in row {
                tile.from_direction = [false; TOTAL_DIRECTIONS];
            }
        }
    }

    fn process_mirror(&mut self, beam: &mut Beam, beam_queue: &mut Vec<Beam>) {
        match self.grid[beam.position.0 as usize][beam.position.1 as usize] {
            '/' => match beam.direction {
                BeamDirection::East => beam.change_direction(BeamDirection::North),
                BeamDirection::North => beam.change_direction(BeamDirection::East),
                BeamDirection::West => beam.change_direction(BeamDirection::South),
                BeamDirection::South => beam.change_direction(BeamDirection::West),
            },
            '\\' => match beam.direction {
                BeamDirection::East => beam.change_direction(BeamDirection::South),
                BeamDirection::North => beam.change_direction(BeamDirection::West),
                BeamDirection::West => beam.change_direction(BeamDirection::North),
                BeamDirection::South => beam.change_direction(BeamDirection::East),
            },
            '|' => {
                if beam.direction == BeamDirection::West || beam.direction == BeamDirection::East {
                    let mut new_beam = beam.clone();
                    new_beam.change_direction(BeamDirection::North);
                    beam_queue.push(new_beam);
                    beam.change_direction(BeamDirection::South);
                }
            }
            '-' => {
                if beam.direction == BeamDirection::North || beam.direction == BeamDirection::South
                {
                    let mut new_beam = beam.clone();
                    new_beam.change_direction(BeamDirection::West);
                    beam_queue.push(new_beam);
                    beam.change_direction(BeamDirection::East);
                }
            }
            _ => (),
        }

        beam.step();
        self.energize(beam, beam_queue);
    }

    fn energize(&mut self, current_beam: &mut Beam, beam_queue: &mut Vec<Beam>) {
        if current_beam.position.0 < 0 || current_beam.position.0 >= self.grid[0].len() as i32 {
            return;
        }
        if current_beam.position.1 < 0 || current_beam.position.1 >= self.grid.len() as i32 {
            return;
        }

        if self.energized[current_beam.position.0 as usize][current_beam.position.1 as usize]
            .from_direction[current_beam.direction.as_idx()]
        {
            return;
        }

        self.energized[current_beam.position.0 as usize][current_beam.position.1 as usize]
            .from_direction[current_beam.direction.as_idx()] = true;
        self.process_mirror(current_beam, beam_queue);
    }

    fn energized_count(&self) -> u32 {
        self.energized
            .iter()
            .map(|v| {
                v.iter()
                    .map(|e| {
                        if e.from_direction.iter().any(|d| *d) {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }

    fn process_beam_queue(&mut self, beam_queue: &mut Vec<Beam>) {
        while let Some(mut current_beam) = beam_queue.pop() {
            self.energize(&mut current_beam, beam_queue);
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day15 <filename>");
        std::process::exit(1);
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut contraption = Contraption::new();

    for line in reader.lines() {
        contraption.add_line(&line?);
    }

    let mut beam_queue: Vec<Beam> = Vec::new();
    beam_queue.push(Beam::new());

    contraption.process_beam_queue(&mut beam_queue);

    let result1: u32 = contraption.energized_count();
    println!("Result for part 1: {}", result1);

    let contraption_h = contraption.grid.len() as i32;
    let contraption_w = contraption.grid[0].len() as i32;

    let mut result2 = result1;
    for i in 0..contraption_h {
        for j in 0..contraption_w {
            let i = i as i32;
            let j = j as i32;
            contraption.reset();
            let mut beam_queue: Vec<Beam> = Vec::new();
            if i == 0 && j == 0 {
                beam_queue.push(Beam::from((i, j), BeamDirection::South));
                contraption.process_beam_queue(&mut beam_queue);
            } else if i == 0 && j == contraption_w {
                beam_queue.push(Beam::from((i, j), BeamDirection::West));
                contraption.process_beam_queue(&mut beam_queue);
                result2 = result2.max(contraption.energized_count());
                contraption.reset();
                beam_queue.push(Beam::from((i, j), BeamDirection::South));
                contraption.process_beam_queue(&mut beam_queue);
            } else if i == contraption_h && j == 0 {
                beam_queue.push(Beam::from((i, j), BeamDirection::East));
                contraption.process_beam_queue(&mut beam_queue);
                contraption.reset();
                result2 = result2.max(contraption.energized_count());
                beam_queue.push(Beam::from((i, j), BeamDirection::North));
                contraption.process_beam_queue(&mut beam_queue);
            } else if i == contraption_h && j == contraption_w {
                beam_queue.push(Beam::from((i, j), BeamDirection::West));
                contraption.process_beam_queue(&mut beam_queue);
                contraption.reset();
                result2 = result2.max(contraption.energized_count());
                beam_queue.push(Beam::from((i, j), BeamDirection::North));
                contraption.process_beam_queue(&mut beam_queue);
            } else if i == 0 {
                beam_queue.push(Beam::from((i, j), BeamDirection::South));
                contraption.process_beam_queue(&mut beam_queue);
            } else if j == 0 {
                beam_queue.push(Beam::from((i, j), BeamDirection::East));
                contraption.process_beam_queue(&mut beam_queue);
            } else if i == contraption_w {
                beam_queue.push(Beam::from((i, j), BeamDirection::North));
                contraption.process_beam_queue(&mut beam_queue);
            } else if j == contraption_w {
                beam_queue.push(Beam::from((i, j), BeamDirection::West));
                contraption.process_beam_queue(&mut beam_queue);
            }
            result2 = result2.max(contraption.energized_count());
        }
    }

    println!("Result for part 2: {}", result2);

    Ok(())
}
