use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn hash(text: &str) -> u32 {
    let mut current_value: u32 = 0;
    for c in text.chars() {
        current_value = ((c as u32 + current_value) * 17) % 256;
    }
    current_value
}

fn parse_line(line: &str) -> Vec<String> {
    let splited = line.split(",");
    splited.map(|s| s.to_string()).collect()
}

#[derive(Debug)]
struct LensSlot {
    box_str: String,
    box_num: u32,
    op: char,
    focal_length: u32,
}

impl LensSlot {
    fn from_str(line: &str) -> Self {
        let mut op: char = '\0';
        let mut box_str = String::new();
        let mut focal_length_str = String::new();
        for c in line.chars() {
            if c == '-' || c == '=' {
                op = c;
            } else if c.is_numeric() {
                focal_length_str.push(c);
            } else {
                box_str.push(c);
            }
        }
        if focal_length_str.is_empty() {
            focal_length_str.push('0');
        }
        LensSlot {
            op: op,
            box_str: box_str.clone(),
            box_num: hash(&box_str),
            focal_length: focal_length_str.parse::<u32>().unwrap(),
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

    let mut steps: Vec<String> = Vec::new();
    for line in reader.lines() {
        steps.extend(parse_line(&line?));
    }
    let result1: u32 = steps.iter().map(|s| hash(s)).sum();
    println!("Result for part 1: {}", result1);

    let mut map: HashMap<u32, Vec<LensSlot>> = HashMap::new();
    steps.iter().for_each(|s| {
        let lens_slot = LensSlot::from_str(s);
        match lens_slot.op {
            '-' => {
                if let Some(lslot_list) = map.get_mut(&lens_slot.box_num) {
                    lslot_list.retain(|s| s.box_str != lens_slot.box_str);
                }            }
            '=' => {
                if let Some(lslot_list) = map.get_mut(&lens_slot.box_num) {
                    let mut changed = false;
                    for slot in lslot_list.iter_mut() {
                        if slot.box_str == lens_slot.box_str {
                            slot.focal_length = lens_slot.focal_length;
                            changed = true;
                        }
                    }
                    if !changed {
                        lslot_list.push(lens_slot);
                    }
                } else {
                    map.entry(lens_slot.box_num)
                        .or_insert_with(Vec::new)
                        .push(lens_slot);
                }
            }
            _ => (),
        };
    });

    let result2: u32 = map.iter().fold(0, |mut acc, (k, v)| {
        let box_val = k + 1;
        for (i, slot) in v.iter().enumerate() {
            let i = i as u32;
            acc += box_val * (i + 1) * slot.focal_length;
        }
        acc
    });

    println!("Result for part 2: {}", result2);

    Ok(())
}
