mod utils;

const INPUT_FILE: &'static str = "./input/day-6-1.in";




fn ways_to_win(t: i64, d: i64) -> i32 {
    let mut ways_to_win = 0;

    for tw in 0..t {
        let dist = (t - tw) * tw;
        // println!("{t} {d} {tw} {dist}");
        if dist > d {
            ways_to_win += 1;
        }
    }

    return ways_to_win;
}

fn main() {
    println!("Processing file: {}", INPUT_FILE);

    let mut time: i64 = 0;
    let mut distance: i64 = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);

                let mut line_in = l.split(':');
                match line_in.next().unwrap() {
                    "Time" => time = line_in.next().unwrap().replace(" ", "").parse().unwrap(),
                    "Distance" => distance = line_in.next().unwrap().replace(" ", "").parse().unwrap(),
                    x => println!("Unknown input: {}", x),
                }
            }
        }
    }

    println!("{}", ways_to_win(time, distance))
}
