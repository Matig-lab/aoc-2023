use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn substr_to_digit(substr: &str, reversed: bool) -> char {
    let digits_nor: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits_rev: [&str; 10] = [
        "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];
    let digits = if reversed { digits_rev } else { digits_nor };
    let mut unmatched = 0;
    for (i, digit_str) in digits.into_iter().enumerate() {
        for (j, c) in substr.chars().enumerate() {
            if digit_str.chars().nth(j).unwrap_or('0') == c {
                if substr.len() == digit_str.len() && substr.eq(digit_str) {
                    return char::from_digit(i as u32, 10).unwrap();
                }
            } else {
                unmatched += 1;
                break;
            }
        }
    }

    return if unmatched == 10 { 'a' } else { 'b' };
}

fn process_line(line: &str) -> i32 {
    let mut substr_start_idx: usize = 0;
    let mut first_num: char = '0';
    let mut last_num: char = '0';
    let line_rev: String = line.chars().rev().collect::<String>();

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            first_num = line.chars().nth(i).unwrap();
            break;
        } else {
            let substr: String = line[substr_start_idx..=i].chars().collect();
            let digit = substr_to_digit(&substr, false);
            if digit.is_digit(10) {
                first_num = digit;
                break;
            } else if digit == 'a' {
                substr_start_idx = i;
            }
        }
    }

    substr_start_idx = 0;
    for (i, c) in line_rev.chars().enumerate() {
        if c.is_digit(10) {
            last_num = line_rev.chars().nth(i).unwrap();
            break;
        } else {
            let substr: String = line_rev[substr_start_idx..=i].chars().collect();
            let digit = substr_to_digit(&substr, true);
            if digit.is_digit(10) {
                last_num = digit;
                break;
            } else if digit == 'a' {
                substr_start_idx += 1;
            }
        }
    }

    let mut num_str: String = String::new();
    num_str.push(first_num);
    num_str.push(last_num);
    num_str.parse().unwrap_or(0)
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
