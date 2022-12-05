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

#[derive(Debug, Clone)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn pop(&mut self) -> T {
        self.stack.pop().expect("No more values to pop!")
    }

    fn pop_n(&mut self, n: usize) -> Vec<T> {
        let mut popped = Vec::new();

        for _ in 0..n {
            match self.stack.pop() {
                Some(val) => {
                    popped.push(val);
                }
                None => continue,
            }
        }

        popped.into_iter().rev().collect()
    }

    fn append(&mut self, items: &mut Vec<T>) {
        self.stack.append(items);
    }
}

fn parse_stacks(input: &str) -> Vec<Stack<char>> {
    let mut stacks: Vec<Stack<char>> = Vec::new();
    let input_lines = input.lines().rev();

    let mut max_num: u32 = 0;

    for (idx, line) in input_lines.enumerate() {
        if idx == 0 {
            println!("first line: {}", line);
            let iter = line.split_whitespace().collect::<Vec<&str>>();
            max_num = iter
                .last()
                .expect("Unable to parse last value!")
                .parse::<u32>()
                .expect("Unable to parse as u32.");

            for _ in 0..max_num {
                stacks.push(Stack::new());
            }
            println!("stacks: {:?}", stacks);

            continue;
        }

        println!("Processing line: {}", line);

        let mut iter = line.chars().into_iter().skip(1).step_by(4);

        for idx in 0..max_num {
            let item = iter.next();
            println!("item: {:?}", item);
            match item {
                Some(container) => {
                    if !container.is_whitespace() {
                        stacks[idx as usize].push(container);
                    }
                }
                None => break,
            }
        }
    }

    println!("stacks: {:?}", stacks);
    stacks
}

fn parse_moveset(moveset: &str) -> (u32, usize, usize) {
    let containers_to_move: u32 = moveset
        .clone()
        .split(" ")
        .skip(1)
        .next()
        .expect("Error parsing number of containers to move!")
        .parse::<u32>()
        .expect("Error parsing as number!");
    let source: usize = moveset
        .clone()
        .split(" ")
        .skip(3)
        .next()
        .expect("Error parsing stack number!")
        .parse::<usize>()
        .expect("Error parsing as number!");
    let destination: usize = moveset
        .clone()
        .split(" ")
        .skip(5)
        .next()
        .expect("Error parsing stack number!")
        .parse::<usize>()
        .expect("Error parsing as number!");

    (containers_to_move, source - 1, destination - 1)
}

fn pop_and_move(n: u32, stacks: &mut Vec<Stack<char>>, src_idx: usize, dest_idx: usize) {
    if n < 1 {
        let val = stacks[src_idx].pop();
        stacks[dest_idx].push(val);
    } else {
        for _ in 0..n {
            let val = stacks[src_idx].pop();
            stacks[dest_idx].push(val);
        }
    }
}

fn solve_first_part(input_file: &str) -> String {
    let mut reader = read_lines(input_file)
        .expect("Error reading file lines...")
        .into_iter()
        .peekable();

    let input = fs::read_to_string(input_file)
        .expect("Error reading file to string")
        .parse::<String>()
        .expect("Error parsing as string.");

    println!("input:\n{}", input);

    let mut splitted = input.split("\n\n");
    let mut stacks = parse_stacks(splitted.next().expect("Can't read containers!"));
    let movesets = splitted.next().expect("Can't read moveset!");

    for moveset in movesets.lines() {
        let (containers_to_move, source_idx, dest_idx) = parse_moveset(moveset);
        println!("===========================================");
        println!("containers_to_move: {}", containers_to_move);
        println!("source stack: {:?}", stacks[source_idx]);
        println!("dest stack: {:?}", stacks[dest_idx]);
        pop_and_move(containers_to_move, &mut stacks, source_idx, dest_idx);
        println!("source stack: {:?}", stacks[source_idx]);
        println!("dest stack: {:?}", stacks[dest_idx]);
        println!("===========================================");
    }

    println!("stacks: {:?}", stacks);

    let mut return_string = "".to_string();
    for mut stack in stacks {
        let val = stack.pop();
        return_string += val.to_string().as_str();
    }

    return_string
}

fn pop_pop_and_move(n: u32, stacks: &mut Vec<Stack<char>>, src_idx: usize, dest_idx: usize) {
    if n < 1 {
        let val = stacks[src_idx].pop();
        stacks[dest_idx].push(val);
    } else {
        let mut popped = stacks[src_idx].pop_n(n as usize);
        stacks[dest_idx].append(&mut popped);
    }
}

fn solve_second_part(input_file: &str) -> String {
    let mut reader = read_lines(input_file)
        .expect("Error reading file lines...")
        .into_iter()
        .peekable();

    let input = fs::read_to_string(input_file)
        .expect("Error reading file to string")
        .parse::<String>()
        .expect("Error parsing as string.");

    println!("input:\n{}", input);

    let mut splitted = input.split("\n\n");
    let mut stacks = parse_stacks(splitted.next().expect("Can't read containers!"));
    let movesets = splitted.next().expect("Can't read moveset!");

    for moveset in movesets.lines() {
        let (containers_to_move, source_idx, dest_idx) = parse_moveset(moveset);
        println!("===========================================");
        println!("containers_to_move: {}", containers_to_move);
        println!("source stack: {:?}", stacks[source_idx]);
        println!("dest stack: {:?}", stacks[dest_idx]);
        pop_pop_and_move(containers_to_move, &mut stacks, source_idx, dest_idx);
        println!("source stack: {:?}", stacks[source_idx]);
        println!("dest stack: {:?}", stacks[dest_idx]);
        println!("===========================================");
    }

    println!("stacks: {:?}", stacks);

    let mut return_string = "".to_string();
    for mut stack in stacks {
        let val = stack.pop();
        return_string += val.to_string().as_str();
    }

    return_string
}
