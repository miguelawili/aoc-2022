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

    let mut stack = vec![("/", 0)];

    let target_amount = 100_000;
    let mut total = 0;

    for line in input.lines() {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir_name = &line[5..];
            if dir_name == ".." {
                let (_, amount) = stack.pop().unwrap();
                if amount <= target_amount {
                    total += amount;
                }
                stack.last_mut().unwrap().1 += amount;
            } else {
                stack.push((dir_name, 0));
            }
            continue;
        }

        let (amount_or_dir, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount_or_dir.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        }

        println!("stack: {:?}", stack);
    }

    println!("total: {}", total);

    total
}

fn solve_second_part(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file)
        .expect("Error reading file to string")
        .parse::<String>()
        .expect("Error parsing as string.");

    let mut stack = vec![("/", 0)];

    let mut counts = vec![];

    let total_space = 70_000_000;
    let install_space_needed = 30_000_000;

    let target_amount = 100_000;
    let mut total = 0;

    for line in input.lines() {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir_name = &line[5..];
            if dir_name == ".." {
                let (name, amount) = stack.pop().unwrap();
                if amount <= target_amount {
                    total += amount;
                }
                stack.last_mut().unwrap().1 += amount;
                counts.push((name, amount))
            } else {
                stack.push((dir_name, 0));
            }
            continue;
        }

        let (amount_or_dir, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount_or_dir.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        }

        println!("stack: {:?}", stack);
    }

    while stack.len() > 0 {
        let (dir_name, amount) = stack.pop().unwrap();
        counts.push((dir_name, amount));

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    let free_space = total_space - counts.last().unwrap().1;
    println!("free_space: {}", free_space);
    let space_required = install_space_needed - free_space;

    let total = counts
        .into_iter()
        .filter(move |(_, amount)| *amount >= space_required)
        .map(|(_, amount)| amount)
        .min()
        .unwrap();

    println!("total: {:?}", total);

    total
}
