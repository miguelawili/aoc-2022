use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // which environment to run (which input to take)
    #[arg(short = 'e', long = "env")]
    environment: String,

    // which part to run?
    #[arg(short = 'p', long = "part")]
    part: String,
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

    if args.part == "1" {
        let ans = solve_first_part(input_file.as_str());

        println!("part1 answer: {:?}", ans);
    } else if args.part == "2" {
        let ans = solve_second_part(input_file.as_str());

        println!("part2 answer: {:?}", ans);
    } else {
        let ans = solve_first_part(input_file.as_str());

        println!("part1 answer: {:?}", ans);

        let ans = solve_second_part(input_file.as_str());

        println!("part2 answer: {:?}", ans);
    }
}

fn solve_first_part(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file)
        .expect("Error reading file to string")
        .parse::<String>()
        .expect("Error parsing as string.");

    let mut curr_idx: usize = 0;
    let num_of_distinct_chars = 4;
    let last_possible_idx = input.len() - num_of_distinct_chars;
    println!("curr_idx: {}", curr_idx);
    println!("last_possible_idx: {}", last_possible_idx);

    while curr_idx < last_possible_idx {
        let seq_last_idx = curr_idx + num_of_distinct_chars;
        let possible_seq = &input[curr_idx..seq_last_idx];

        let mut iter = possible_seq.chars().peekable();
        let mut letters: HashSet<char> = HashSet::new();

        while !iter.peek().is_none() {
            let current_letter = iter.next().expect("Impossible there's no value!");

            match letters.get(&current_letter) {
                _ => letters.insert(current_letter),
            };
        }

        println!("letters: {:?}", letters);

        if letters.len() == num_of_distinct_chars {
            break;
        } else {
            curr_idx += 1;
        }
    }

    curr_idx + num_of_distinct_chars
}

fn solve_second_part(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file)
        .expect("Error reading file to string")
        .parse::<String>()
        .expect("Error parsing as string.");

    let mut curr_idx: usize = 0;
    let num_of_distinct_chars = 14;
    let last_possible_idx = input.len() - num_of_distinct_chars;
    println!("curr_idx: {}", curr_idx);
    println!("last_possible_idx: {}", last_possible_idx);

    while curr_idx < last_possible_idx {
        let seq_last_idx = curr_idx + num_of_distinct_chars;
        let possible_seq = &input[curr_idx..seq_last_idx];

        let mut iter = possible_seq.chars().peekable();
        let mut letters: HashSet<char> = HashSet::new();

        while !iter.peek().is_none() {
            let current_letter = iter.next().expect("Impossible there's no value!");

            match letters.get(&current_letter) {
                _ => letters.insert(current_letter),
            };
        }

        println!("letters: {:?}", letters);

        if letters.len() == num_of_distinct_chars {
            break;
        } else {
            curr_idx += 1;
        }
    }

    curr_idx + num_of_distinct_chars
}
