use std::cmp::max;

mod utils;

const INPUT_FILE: &'static str = "./input/day-2-2.in";

fn process_line(line: &str) -> i32 {
    let mut game_split = line.split(':');
    let _ = game_split.nth(0).unwrap().split(' ').nth(1).unwrap().parse().unwrap_or(0);

    let mut red= 0;
    let mut green= 0;
    let mut blue= 0;

    for g in  game_split.nth(0).unwrap().split(';') {
        for color_split in g.split(',') {
            println!("{}", color_split.strip_prefix(' ').unwrap());
            let mut color_tuple = color_split.strip_prefix(' ').unwrap().split(' ');
            let color_num = color_tuple.nth(0).unwrap().parse().unwrap_or(0);
            match color_tuple.nth(0).unwrap() {
                "red" => red = max(red, color_num),
                "green" => green = max(green, color_num),
                "blue" => blue = max(blue, color_num),
                c => println!("unexpected color {}", c)
            }
        }
    }

    return red * green * blue;
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
                sum += process_line(&l);
            }
        }
    }

    println!("{}", sum)
}
