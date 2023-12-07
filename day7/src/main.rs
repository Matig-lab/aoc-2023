use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    #[allow(dead_code)]
    fn to_string(self) -> String {
        match self {
            HandType::None => String::from("None"),
            HandType::HighCard => String::from("High Card"),
            HandType::OnePair => String::from("One Pair"),
            HandType::TwoPair => String::from("Two Pair"),
            HandType::ThreeOfAKind => String::from("Three of a Kind"),
            HandType::FullHouse => String::from("Full House"),
            HandType::FourOfAKind => String::from("Four of a Kind"),
            HandType::FiveOfAKind => String::from("Five of a Kind"),
        }
    }

    fn get_hand_type(highest_count: u32, second_highest_count: u32) -> HandType {
        match highest_count {
            1 => HandType::HighCard,
            2 => {
                if second_highest_count == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            3 => {
                if second_highest_count == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            _ => HandType::None,
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    id: u32,
    cards: String,
    bid: u32,
    rank: u32,
    hand_type: HandType,
    hand_type_with_joker: HandType,
}

const CARD_VALUES_SIZE: usize = 265;
const CARD_VALUES: [u32; CARD_VALUES_SIZE] = {
    let mut values = [0; CARD_VALUES_SIZE];
    values['A' as usize] = 14;
    values['K' as usize] = 13;
    values['Q' as usize] = 12;
    values['J' as usize] = 11;
    values['T' as usize] = 10;
    values['9' as usize] = 9;
    values['8' as usize] = 8;
    values['7' as usize] = 7;
    values['6' as usize] = 6;
    values['5' as usize] = 5;
    values['4' as usize] = 4;
    values['3' as usize] = 3;
    values['2' as usize] = 2;
    values
};

impl Hand {
    fn new() -> Hand {
        Hand {
            id: 0,
            cards: String::new(),
            bid: 0,
            rank: 0,
            hand_type: HandType::None,
            hand_type_with_joker: HandType::None,
        }
    }

    fn new_from_line(line: &str, id: u32) -> Hand {
        let mut splited_line = line.split_whitespace();
        let mut hand = Hand::new();
        hand.id = id;
        hand.cards = splited_line.next().unwrap().to_string();
        hand.bid = splited_line.next().unwrap().parse::<u32>().unwrap();
        hand.classify_type();
        hand
    }

    fn classify_type(&mut self) {
        let mut value_counts: HashMap<char, usize> = HashMap::new();

        for card in self.cards.chars() {
            let count = value_counts.entry(card).or_insert(0);
            *count += 1;
        }

        let mut highest_count: u32 = 0;
        let mut second_highest_count: u32 = 0;

        let mut highest_count_j: u32 = 0;
        let mut second_highest_count_j: u32 = 0;

        value_counts.iter().for_each(|(&card, &count)| {
            let count_u32 = count as u32;

            if count_u32 > highest_count {
                second_highest_count = highest_count;
                highest_count = count_u32;
            } else if count_u32 > second_highest_count {
                second_highest_count = count_u32;
            }
            if card != 'J' && count_u32 > highest_count_j {
                second_highest_count_j = highest_count_j;
                highest_count_j = count_u32;
            } else if card != 'J' && count_u32 > second_highest_count_j {
                second_highest_count_j = count_u32;
            }
        });

        let highest_with_joker =
            highest_count_j + value_counts.get(&'J').map_or(0, |&count| count as u32);

        self.hand_type_with_joker =
            HandType::get_hand_type(highest_with_joker, second_highest_count_j);
        self.hand_type = HandType::get_hand_type(highest_count, second_highest_count);
    }

    fn is_stronger_than(&self, other: &Hand, j_is_joker: bool) -> bool {
        let self_hand_type: &HandType = if j_is_joker {
            &self.hand_type_with_joker
        } else {
            &self.hand_type
        };
        let other_hand_type: &HandType = if j_is_joker {
            &other.hand_type_with_joker
        } else {
            &other.hand_type
        };

        if *self_hand_type as i32 != *other_hand_type as i32 {
            return *self_hand_type as i32 > *other_hand_type as i32;
        }

        for (self_card, other_card) in self.cards.chars().zip(other.cards.chars()) {
            if self_card == other_card {
                continue;
            }

            let self_card_val = if j_is_joker && self_card == 'J' {
                1
            } else {
                CARD_VALUES[self_card as usize]
            };
            let other_card_val = if j_is_joker && other_card == 'J' {
                1
            } else {
                CARD_VALUES[other_card as usize]
            };
            return self_card_val > other_card_val;
        }
        false
    }

    fn assign_rank(&mut self, rank: u32) {
        self.rank = rank;
    }

    fn compare(&self, other: &Hand, j_is_joker: bool) -> Ordering {
        let self_hand_type: HandType = if j_is_joker {
            self.hand_type_with_joker
        } else {
            self.hand_type
        };
        let other_hand_type: HandType = if j_is_joker {
            other.hand_type_with_joker
        } else {
            other.hand_type
        };

        if self_hand_type != other_hand_type {
            self_hand_type.cmp(&other_hand_type)
        } else {
            self.is_stronger_than(other, j_is_joker).cmp(&true)
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ./day7 <filename>");
        std::process::exit(1);
    }
    let f = File::open(&args[1])?;
    let reader = BufReader::new(f);

    let mut hands: Vec<Hand> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        hands.push(Hand::new_from_line(&line?, i as u32));
    }

    hands.sort_by(|a, b| a.compare(b, false));

    for (index, hand) in hands.iter_mut().enumerate() {
        hand.assign_rank(index as u32 + 1);
    }

    let result1 = hands.iter().fold(0, |mut acc, hand| {
        acc += hand.bid * hand.rank;
        acc
    });

    hands.sort_by(|a, b| a.compare(b, true));

    for (index, hand) in hands.iter_mut().enumerate() {
        hand.assign_rank(index as u32 + 1);
    }

    let result2 = hands.iter().fold(0, |mut acc, hand| {
        acc += hand.bid * hand.rank;
        acc
    });

    println!("Result for part 1: {}", result1);
    println!("Result for part 2: {}", result2);

    Ok(())
}
