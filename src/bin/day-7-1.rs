use std::collections::HashMap;

mod utils;

const INPUT_FILE: &'static str = "./input/day-7-1.in";

fn card_value(c: &char) -> char {
    match c {
        'A' => return 'm',
        'K' => return 'l',
        'Q' => return 'k',
        'J' => return 'j',
        'T' => return 'i',
        '9' => return 'h',
        '8' => return 'g',
        '7' => return 'f',
        '6' => return 'e',
        '5' => return 'd',
        '4' => return 'c',
        '3' => return 'b',
        '2' => return 'a',
        _ => panic!("unknown card value")
    }
}

fn cards_to_value_repr(cards: &str) -> String {
    let mut groups: HashMap<char, i32> = HashMap::new();
    for c in cards.chars() {
        *groups.entry(c).or_insert(0) += 1;
    }
    let mut card_val = vec![' ', ' ', ' ', ' ', ' ', ' '];
    println!("{}", serde_json::to_string(&groups).unwrap());
    match groups.len() {
        1 => { card_val[0] = 'g'; } // five of a kind
        2 => {
            if *groups.values().max().unwrap() == 4 {
                card_val[0] = 'f';  // four of a kind
            } else {
                card_val[0] = 'e'; // full house
            }
        }
        3 => {
            if *groups.values().max().unwrap() == 3 {
                card_val[0] = 'd';  // three of a kind
            } else {
                card_val[0] = 'c'; // two pair
            }
        }
        4 => { card_val[0] = 'b'; } // pair
        5 => { card_val[0] = 'a'; } // High card
        _ => panic!("more than 5 groups of cards")
    }

    let mut rank = 1;
    for c in cards.chars() {
        card_val[rank] = card_value(&c);
        rank += 1;
    }

    println!("{}", card_val.iter().collect::<String>());

    return card_val.iter().collect();
}

fn main() {
    println!("Processing file: {}", INPUT_FILE);

    let mut cards_bets: Vec<(String, i32)> = vec![];

    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);

                let mut line_in = l.split(' ');
                cards_bets.push((line_in.next().unwrap().to_string(), line_in.next().unwrap().parse().unwrap()));
            }
        }
    }

    cards_bets.sort_by(|(l, _), (r, _)| cards_to_value_repr(l).partial_cmp(&cards_to_value_repr(r)).unwrap());

    let mut value = 0i64;
    let mut rank = 1;
    for (i, j) in &cards_bets {
        println!("{i}, {j}, {}", cards_to_value_repr(i));
        value += rank as i64 * *j as i64;
        rank += 1;
    }

    println!("{}", value)
}
