use std::collections::HashSet;

mod utils;

const INPUT_FILE: &'static str = "./input/day-4-1.in";
// const INPUT_FILE: &'static str = "./input/day-4-1-small.in";

fn wins(line: &str) -> i32 {
    let mut line_split = line.split(':');

    let _ = line_split.next();

    let mut numbers = line_split.next().unwrap().split("|");
    let ticket = numbers.next().unwrap().split(" ");
    let lottery = numbers.next().unwrap().split(" ");

    let mut winning: HashSet<i32> = HashSet::new();
    let mut wins = 0;

    for i in lottery {
        if i == "" {
            continue;
        }

        winning.insert(i.parse().unwrap());
    }

    for i in ticket {
        if i == "" {
            continue;
        }

        if winning.contains(&i.parse().unwrap()) {
            wins += 1;
        }
    }

    return wins;
}

fn main() {
    let mut sum = 0;

    println!("Processing file: {}", INPUT_FILE);

    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
                let w = wins(&l);
                if w > 0 {
                    sum += i32::pow(2, w as u32 - 1);
                }
            }
        }
    }

    println!("{}", sum)
}
