mod utils;

const INPUT_FILE: &'static str = "./input/day-2-1.in";

fn process_line(line: &str) -> i32 {
    let mut game_split = line.split(':');
    let game_id = game_split.nth(0).unwrap().split(' ').nth(1).unwrap().parse().unwrap_or(0);

    for g in  game_split.nth(0).unwrap().split(';') {
        let mut red= 0;
        let mut green= 0;
        let mut blue= 0;

        for color_split in g.split(',') {
            println!("{}", color_split.strip_prefix(' ').unwrap());
            let mut color_tuple = color_split.strip_prefix(' ').unwrap().split(' ');
            let color_num = color_tuple.nth(0).unwrap().parse().unwrap_or(0);
            match color_tuple.nth(0).unwrap() {
                "red" => red += color_num,
                "green" => green += color_num,
                "blue" => blue += color_num,
                c => println!("unexpected color {}", c)
            }
        }

        if red > 12 || green > 13 || blue > 14 {
            return 0;
        }
    }

    return game_id;
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
