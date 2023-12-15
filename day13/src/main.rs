use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
    width: i32,
    height: i32,
}

impl Pattern {
    fn new() -> Self {
        Pattern {
            rows: Vec::new(),
            cols: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn from(pattern_str: &str, width: i32) -> Self {
        let mut p = Pattern::new();
        let mut first_iteration = true;
        p.width = width;
        for i in 0..width {
            let i = i as usize;
            let mut vert_str = String::new();
            for line in pattern_str.lines() {
                if first_iteration {
                    if !line.trim().is_empty() {
                        p.rows.push(line.to_string());
                        p.height += 1;
                    }
                }
                if let Some(c) = line.chars().nth(i) {
                    vert_str.push(c);
                }
            }
            p.cols.push(vert_str);
            first_iteration = false;
        }
        p
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum SymmetryType {
    VerticalSymmetry,
    HorizontalSymmetry,
}

fn process_symmetry_value(val: (i32, SymmetryType)) -> i32 {
    if val.1 == SymmetryType::HorizontalSymmetry {
        val.0 * 100
    } else {
        val.0
    }
}

fn diff(str1: &str, str2: &str) -> i32 {
    str1.chars()
        .zip(str2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
        .try_into()
        .unwrap()
}

fn check_mirrors(symmetry_axis: &Vec<String>, symmetry_center: usize, smudge: bool) -> i32 {
    let mut back: i32 = symmetry_center as i32;
    let mut front: i32 = symmetry_center as i32 + 1;
    let axis_len = symmetry_axis.len() as i32;
    let mut smudges = 0;
    let mut count = 0;
    while back >= 0 && front < axis_len {
        let front_str = &symmetry_axis[front as usize];
        let back_str = &symmetry_axis[back as usize];

        if front_str == back_str {
            count += 1
        } else if smudge && smudges < 1 {
            let diff_count = diff(&back_str, &front_str);
            if diff_count == 1 {
                smudges += 1;
                count += 1;
            } else {
                return 0;
            }
        } else {
            return 0;
        }
        back -= 1;
        front += 1;
    }
    if smudge {
        if smudges == 1 {
            return count;
        } else {
            return 0;
        }
    }
    count
}

fn process_perfect_symmetry(pattern: &Pattern, smudge: bool) -> (i32, SymmetryType) {
    let mut last_row = String::default();
    let mut mirrored_rows = 0i32;
    let mut last_row_sum = 0;
    for (i, row) in pattern.rows.iter().enumerate() {
        let i = i as i32;
        if last_row == *row || (smudge && diff(&last_row, &row) == 1) {
            let current_sum = check_mirrors(&pattern.rows, (i - 1).try_into().unwrap(), smudge);
            if current_sum > last_row_sum {
                mirrored_rows = i - 1;
                last_row_sum = current_sum;
            }
        }
        last_row = row.to_string();
    }

    let mut last_col = String::default();
    let mut mirrored_cols = 0i32;
    let mut last_col_sum = 0;
    for (i, col) in pattern.cols.iter().enumerate() {
        let i = i as i32;
        if last_col == *col || (smudge && diff(&last_col, &col) == 1) {
            let current_sum = check_mirrors(&pattern.cols, (i - 1).try_into().unwrap(), smudge);
            if current_sum > last_row_sum {
                mirrored_cols = i - 1;
                last_col_sum = current_sum;
            }
        }
        last_col = col.to_string();
    }

    mirrored_cols += 1;
    mirrored_rows += 1;

    if last_col_sum > last_row_sum {
        (mirrored_cols, SymmetryType::VerticalSymmetry)
    } else {
        (mirrored_rows, SymmetryType::HorizontalSymmetry)
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day13 <filename>");
        std::process::exit(1);
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut current_pattern = String::new();
    let mut patterns: Vec<Pattern> = Vec::new();

    let mut current_width = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        current_width = if line.len() != 0 {
            line.len() as i32
        } else {
            current_width
        };
        current_pattern.push_str(&line);
        current_pattern.push('\n');
        if line.trim().is_empty() {
            patterns.push(Pattern::from(&current_pattern, current_width));
            current_pattern.clear();
        }
    }

    if !current_pattern.trim().is_empty() {
        patterns.push(Pattern::from(&current_pattern, current_width));
        current_pattern.clear();
    }

    let result1: i32 = patterns
        .iter()
        .map(|p| {
            let sym = process_perfect_symmetry(p, false);
            process_symmetry_value(sym)
        })
        .sum();

    let result2: i32 = patterns
        .iter()
        .map(|p| {
            let sym = process_perfect_symmetry(p, true);
            process_symmetry_value(sym)
        })
        .sum();

    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
