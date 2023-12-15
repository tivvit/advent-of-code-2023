use std::collections::HashMap;

mod utils;

const INPUT_FILE: &'static str = "./input/day-7-1.in";
// const INPUT_FILE: &'static str = "./input/day-7-1-small.in";

fn card_value(c: &char) -> char {
    match c {
        'A' => return 'm',
        'K' => return 'l',
        'Q' => return 'k',
        'T' => return 'j',
        '9' => return 'i',
        '8' => return 'h',
        '7' => return 'g',
        '6' => return 'f',
        '5' => return 'e',
        '4' => return 'd',
        '3' => return 'c',
        '2' => return 'b',
        'J' => return 'a',
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
    // find best without jokers
    let mut best = 0;
    let mut best_card = ' ';
    for (k, v) in groups {
        if k != 'J' && v > best {
            best = v;
            best_card = k;
        }
    }
    let improved_cards = cards.replace('J', &best_card.to_string());
    let mut improved_groups: HashMap<char, i32> = HashMap::new();
    for c in improved_cards.chars() {
        *improved_groups.entry(c).or_insert(0) += 1;
    }
    println!("{}", serde_json::to_string(&improved_groups).unwrap());
    match improved_groups.len() {
        1 => { card_val[0] = 'g'; } // five of a kind
        2 => {
            if *improved_groups.values().max().unwrap() == 4 {
                card_val[0] = 'f';  // four of a kind
            } else {
                card_val[0] = 'e'; // full house
            }
        }
        3 => {
            if *improved_groups.values().max().unwrap() == 3 {
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
