use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // which environment to run (which input to take)
    #[arg(short = 'e', long = "env")]
    environment: String,
}

fn read_lines(filepath: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let args = Args::parse();

    let input_file: String;

    if args.environment == "prod" {
        input_file = "input.prod".to_string();
    } else {
        input_file = "input.test".to_string();
    }

    solve_first_part(input_file.as_str());
    solve_second_part(input_file.as_str());
}

fn get_character_priority(letter: char) -> u32 {
    // dbg!("get_character_priority() letter: {}", letter);
    let priority: u32;
    let ascii_equivalent = letter as u32;

    if letter.is_ascii_lowercase() {
        priority = ascii_equivalent - 96;
    } else if letter.is_uppercase() {
        priority = ascii_equivalent - 38;
    } else {
        panic!(
            "Can't read character! letter: {}; ascii: {};",
            letter, ascii_equivalent
        );
    }

    priority
}

fn split_half(string: &str) -> (Vec<char>, Vec<char>) {
    let mid_pt: usize = string.len() / 2;
    let (first, second) = string.split_at(mid_pt);
    if first.len() == second.len() {
        println!("Both halves are equal!");
    } else {
        println!("Unequal! first: {}; second: {}", first.len(), second.len());
    }

    (first.chars().collect(), second.chars().collect())
}

fn solve_first_part(input_file: &str) {
    let mut priorities_sum: u32 = 0;

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            match line {
                Ok(rucksack) => {
                    println!("rucksacks: {}", rucksack);
                    let rucksack = rucksack.trim();

                    let (first_rucksack, second_rucksack) = split_half(rucksack);

                    let mut first_rucksack_map: HashMap<char, u8> = HashMap::new();

                    // println!("first_rucksack: {:?}", first_rucksack);
                    // println!("second_rucksack: {:?}", second_rucksack);

                    for letter in &first_rucksack {
                        first_rucksack_map.insert(*letter, 1);
                    }

                    // println!("first_rucksack_map: {:#?}", first_rucksack_map);
                    let mut matching_letter: char = char::default();

                    for letter in &second_rucksack {
                        // dbg!("{}", first_rucksack_map.get_key_value(letter));
                        match first_rucksack_map.get_key_value(letter) {
                            Some((letter, _)) => matching_letter = *letter,
                            None => continue,
                        };
                    }

                    let character_priority: u32 = get_character_priority(matching_letter);
                    println!("matching letter: {}", matching_letter);
                    println!("matching letter priority: {}", character_priority);

                    priorities_sum += character_priority;
                }
                _ => {}
            }
        }
    };

    println!("sum of priorities: {}", priorities_sum);
}

fn hashset(chars: Vec<char>) -> HashSet<char> {
    HashSet::from_iter(chars.iter().cloned())
}

fn solve_second_part(input_file: &str) {
    let mut reader = read_lines(input_file)
        .expect("Error reading file lines...")
        .into_iter()
        .peekable();

    let mut priorities_sum: u32 = 0;

    while !reader.peek().is_none() {
        let first: String;
        let second: String;
        let third: String;

        match reader.next() {
            Some(rucksack) => match rucksack {
                Ok(rucksack) => first = rucksack,
                Err(err) => panic!("Missing first rucksack! {}", err),
            },
            None => panic!("Missing first rucksack!"),
        }
        match reader.next() {
            Some(rucksack) => match rucksack {
                Ok(rucksack) => second = rucksack,
                Err(err) => panic!("Missing second rucksack! {}", err),
            },
            None => panic!("Missing second rucksack!"),
        }
        match reader.next() {
            Some(rucksack) => match rucksack {
                Ok(rucksack) => third = rucksack,
                Err(err) => panic!("Missing third rucksack! {}", err),
            },
            None => panic!("Missing third rucksack!"),
        }

        println!("grouped: ");

        println!("actual: {:?}", first);
        println!("actual: {:?}", second);
        println!("actual: {:?}", third);

        let vec1: Vec<char> = first.chars().collect();
        let vec2: Vec<char> = second.chars().collect();
        let vec3: Vec<char> = third.chars().collect();

        let set1 = hashset(vec1);
        let set2 = hashset(vec2);
        let set3 = hashset(vec3);
        let mut combined_sets: Vec<HashSet<char>> = Vec::new();
        combined_sets.push(set1.clone());
        combined_sets.push(set2.clone());
        combined_sets.push(set3.clone());
        // println!("set1: {:?}", set1);
        // println!("set2: {:?}", set2);
        // println!("set3: {:?}", set3);
        // println!("combined_sets: {:?}", combined_sets);

        let inner_joined = combined_sets
            .iter()
            .skip(1)
            .fold(combined_sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });

        println!("intersection: {:?}", &inner_joined);

        let common_char = inner_joined
            .into_iter()
            .next()
            .expect("No value in hashmap!");

        let to_add: u32 = get_character_priority(common_char);

        priorities_sum += to_add;
    }

    println!("priorities_sum: {}", priorities_sum);
}
