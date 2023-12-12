use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_arragements(
    spring_row: &str,
    blocks: &Vec<u64>,
    spring_idx: usize,
    blocks_idx: usize,
    current_block_pos: usize,
    hash_table: &mut HashMap<(String, Vec<u64>, usize, usize, usize), u64>,
) -> u64 {
    let table_entry = (
        spring_row.to_string(),
        blocks.clone(),
        spring_idx,
        blocks_idx,
        current_block_pos,
    );

    if let Some(cached) = hash_table.get(&table_entry) {
        return *cached;
    }

    if spring_idx == spring_row.len() {
        if blocks_idx == blocks.len() && current_block_pos == 0 {
            return 1;
        } else if blocks_idx == blocks.len() - 1
            && blocks[blocks_idx] == current_block_pos.try_into().unwrap()
        {
            // There is a block at last
            return 1;
        } else {
            return 0;
        }
    }

    let mut arrangements = 0;
    let current_char = spring_row.chars().nth(spring_idx).unwrap();

    for c in ['.', '#'] {
        if current_char == c || current_char == '?' {
            // Send .
            if c == '.' && current_block_pos == 0 {
                arrangements += calc_arragements(
                    spring_row,
                    blocks,
                    spring_idx + 1,
                    blocks_idx,
                    0,
                    hash_table,
                );
            } else if c == '.'
                && current_block_pos > 0
                && blocks_idx < blocks.len()
                && blocks[blocks_idx] == current_block_pos as u64
            {
                arrangements += calc_arragements(
                    spring_row,
                    blocks,
                    spring_idx + 1,
                    blocks_idx + 1,
                    0,
                    hash_table,
                );
            // Send #
            } else if c == '#' {
                arrangements += calc_arragements(
                    spring_row,
                    blocks,
                    spring_idx + 1,
                    blocks_idx,
                    current_block_pos + 1,
                    hash_table,
                );
            }
        }
    }

    hash_table.insert(table_entry, arrangements);
    arrangements
}

fn parse_line(line: &str) -> (String, Vec<u64>) {
    let mut splited = line.split_whitespace();
    let spring_row = splited.next().unwrap().to_string();
    let blocks = splited
        .next()
        .unwrap()
        .split(",")
        .map(|c| c.parse::<u64>().unwrap())
        .collect();

    (spring_row, blocks)
}

fn spring_extend(spring_row: &str, blocks: &Vec<u64>) -> (String, Vec<u64>) {
    let mut extended_spring = spring_row.to_string();
    let mut extended_blocks = blocks.clone();

    for _i in 0..4 {
        extended_spring.push('?');
        extended_spring.push_str(spring_row);
        extended_blocks.extend(blocks);
    }

    (extended_spring, extended_blocks)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day12 <filename>");
        std::process::exit(1);
    }

    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut sum_of_arrangements1 = 0;
    let mut sum_of_arrangements2 = 0;
    let mut hash_table: HashMap<(String, Vec<u64>, usize, usize, usize), u64> = HashMap::new();
    for line in reader.lines() {
        let (spring_row, blocks) = parse_line(&line?);
        let arragements1 = calc_arragements(&spring_row, &blocks, 0, 0, 0, &mut hash_table);
        sum_of_arrangements1 += arragements1;

        hash_table.clear();
        let (extended_spring, extended_block) = spring_extend(&spring_row, &blocks);
        let arragements2 = calc_arragements(&extended_spring, &extended_block, 0, 0, 0, &mut hash_table);
        sum_of_arrangements2 += arragements2;
    }

    let result1 = sum_of_arrangements1;
    let result2 = sum_of_arrangements2;
    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
