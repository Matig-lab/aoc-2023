use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn diff_vec_of(numbers: &Vec<i32>) -> Vec<i32> {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .map(|(a, b)| (b - a) as i32)
        .collect::<Vec<_>>()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect()
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day9 <filename>");
        std::process::exit(1);
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut histories: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        histories.push(parse_line(&line?));
    }

    let predictions: Vec<(i32, i32)> = histories
        .iter()
        .map(|h| {
            let mut diffs: Vec<Vec<i32>> = Vec::new();
            let mut current: &Vec<i32> = &h;

            loop {
                let diff = diff_vec_of(current);
                diffs.push(diff.clone());
                if diff.iter().all(|n| *n == 0) {
                    break;
                }
                current = diffs.last().unwrap();
            }

            let prediction_right = diffs.iter().fold(0 as i32, |mut acc, d| {
                acc += d.last().unwrap();
                acc
            }) + h.last().unwrap();

            let prediction_left = -(diffs.iter().rev().fold(0 as i32, |mut acc, d| {
                acc = -acc + d.first().unwrap();
                acc
            })) + h.first().unwrap();

            (prediction_left, prediction_right)
        })
        .collect();

    let (result1, result2) = predictions
        .iter()
        .fold((0, 0), |(acc1, acc2), p| (acc1 + p.1, acc2 + p.0));

    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
