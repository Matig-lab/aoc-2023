use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn time_pressed_from_distance(distance: i64, time_limit: i64) -> Result<(f64, f64), &'static str> {
    let discriminant: i128 = ((time_limit * time_limit) - (4 * distance))
        .try_into()
        .unwrap();

    if discriminant >= 0 {
        let sqrt_discriminant = (discriminant as f64).sqrt();
        let t1 = -(-time_limit as f64 + sqrt_discriminant) / 2.0;
        let t2 = -(-time_limit as f64 - sqrt_discriminant) / 2.0;
        Ok((t1.floor(), t2.ceil()))
    } else {
        Err("No real roots")
    }
}

fn parse_line(line: &str) -> Vec<u64> {
    let line_splited = line.split(":");
    line_splited
        .last()
        .unwrap()
        .split_whitespace()
        .map(|time| time.trim().parse::<u64>().unwrap())
        .collect()
}

fn join_to_number(vec: Vec<u64>) -> i64 {
    let joined_string: String = vec.iter().map(|&num| num.to_string()).collect();
    let joined_number: i64 = joined_string.parse().unwrap_or(0);
    joined_number
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day6 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let lines = &mut reader.lines();
    let times = parse_line(&lines.next().unwrap().unwrap());
    let records = parse_line(&lines.next().unwrap().unwrap());

    let mut result1: u64 = 1;
    for i in 0..times.len() {
        let extremes = time_pressed_from_distance(
            records[i].try_into().unwrap(),
            times[i].try_into().unwrap(),
        )
        .unwrap();
        let possibles = extremes.1 - extremes.0 - 1.0;
        result1 *= possibles as u64;
    }

    let big_time_limit = join_to_number(times);
    let big_record_meters = join_to_number(records);
    let extremes_of_single_race =
        time_pressed_from_distance(big_record_meters, big_time_limit).unwrap();

    let result2 = extremes_of_single_race.1 - extremes_of_single_race.0 - 1.0;

    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
