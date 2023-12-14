use std::collections::{HashMap, HashSet};

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
    println!("Processing file: {}", INPUT_FILE);

    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    let mut card = 1;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
                // self
                *occurrences.entry(card).or_insert(0) += 1;
                let o: i32 = occurrences[&card];
                // following + self
                for i in card+1..card+wins(&l)+1 {
                    *occurrences.entry(i).or_insert(0) += o;
                }
                println!("{}", serde_json::to_string(&occurrences).unwrap());
                card += 1;
            }
        }
    }

    for (k,v) in &occurrences {
        println!("{k}: {v}")
    }

    println!("{}", occurrences.values().sum::<i32>())
}
