use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_line(line: &str) -> i32 {
    let mut f_idx: i32 = -1; // First digit index let mut s_idx: i32 = -1; // Second
    let mut l_idx: i32 = -1;
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            if f_idx == -1 {
                f_idx = i as i32;
            } else {
                l_idx = i as i32;
            }
        }
    }
    let mut num_str: String = String::new();
    if f_idx >= 0 {
        num_str.push(line.chars().nth(f_idx as usize).unwrap());
    }
    if l_idx >= 0 {
        num_str.push(line.chars().nth(l_idx as usize).unwrap());
    } else {
        num_str.push(line.chars().nth(f_idx as usize).unwrap());
    }
    num_str.parse().unwrap()
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day1 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut val = 0;
    for line in reader.lines() {
        let c = process_line(&line?);
        val += c;
    }

    println!("Result: {}", val);

    Ok(())
}
