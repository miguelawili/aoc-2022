use std::fs::File;
use std::io;
use std::io::BufRead;

fn read_lines(filepath: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut elfs: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        let mut calories_count: u32 = 0;

        for line in lines {
            match line {
                Ok(count) => match count.parse::<u32>() {
                    Ok(to_add) => {
                        dbg!("to_add: {}", to_add);
                        calories_count += to_add;
                    }
                    _ => {
                        dbg!("elf calories count: {}", calories_count);
                        elfs.push(calories_count);
                        dbg!("Resetting count to 0");
                        calories_count = 0;
                        dbg!("elf calories count: {}", calories_count);
                        continue;
                    }
                },
                _ => {}
            }
        }
    }

    println!("all elfs: {:#?}", elfs);
    elfs.sort();
    println!("all elfs (sorted): {:#?}", elfs);

    let last_element_idx = elfs.len() - 1;
    let mut top_three_total: u32 = 0;
    for entry in &elfs[(last_element_idx - 2)..] {
        top_three_total += entry;
    }

    println!("elf with most calories: {:#?}", &elfs[last_element_idx]);
    println!("top three elfs total: {:#?}", top_three_total);
}
