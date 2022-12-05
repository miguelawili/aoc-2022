use clap::Parser;
use std::collections::HashSet;
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

        println!("part1 answer: {}", ans);
    } else if args.part == "2" {
        let ans = solve_second_part(input_file.as_str());

        println!("part2 answer: {}", ans);
    } else {
        let ans = solve_first_part(input_file.as_str());

        println!("part1 answer: {}", ans);

        let ans = solve_second_part(input_file.as_str());

        println!("part2 answer: {}", ans);
    }
}

fn range_to_set(range: &str) -> HashSet<String> {
    let mut whole = range.split("-");
    let min_str = whole.next().expect("No minimum!");
    let max_str = whole.next().expect("No maximum!");

    let min = min_str.parse::<u32>().expect("Error parsing as u32.");
    let max = max_str.parse::<u32>().expect("Error parsing as u32.");

    let mut sections: HashSet<String> = HashSet::new();

    for num in min..max + 1 {
        sections.insert(num.to_string());
    }

    sections
}

fn solve_first_part(input_file: &str) -> u32 {
    let mut reader = read_lines(input_file)
        .expect("Error reading file lines...")
        .into_iter()
        .peekable();

    let mut pairs: u32 = 0;

    while !reader.peek().is_none() {
        let line = reader
            .next()
            .expect("No more lines!")
            .expect("Error parsing as ascii string!");

        let mut splitted = line.split(",");
        let section1 = splitted.next().expect("No section1!");
        let section2 = splitted.next().expect("No section2!");

        println!("raw section1: {}", section1);
        println!("raw section2: {}", section2);

        let section1 = range_to_set(section1);
        let section2 = range_to_set(section2);

        println!("set section1: {:?}", section1);
        println!("set section2: {:?}", section2);

        if section1.len() == section2.len() {
            if section1 == section2 {
                pairs += 1;
            }
        } else if section1.len() < section2.len() {
            if section1.is_subset(&section2) {
                pairs += 1;
            }
        } else {
            if section2.is_subset(&section1) {
                pairs += 1;
            }
        }
    }

    pairs
}

fn solve_second_part(input_file: &str) -> u32 {
    let mut reader = read_lines(input_file)
        .expect("Error reading file lines...")
        .into_iter()
        .peekable();

    let mut pairs: u32 = 0;

    while !reader.peek().is_none() {
        let line = reader
            .next()
            .expect("No more lines!")
            .expect("Error parsing as ascii string!");

        let mut splitted = line.split(",");
        let section1 = splitted.next().expect("No section1!");
        let section2 = splitted.next().expect("No section2!");

        println!("raw section1: {}", section1);
        println!("raw section2: {}", section2);

        let section1 = range_to_set(section1);
        let section2 = range_to_set(section2);

        println!("set section1: {:?}", section1);
        println!("set section2: {:?}", section2);

        let mut intersecting = section1.intersection(&section2).peekable();

        if !intersecting.peek().is_none() {
            println!("intersecting: {:?}", &intersecting);
            pairs += 1;
        }
    }

    pairs
}
