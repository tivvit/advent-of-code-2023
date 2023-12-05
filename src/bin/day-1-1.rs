mod day_1;
mod utils;

const INPUT_FILE: &'static str = "./input/day-1-1.in";

fn main() {
    let mut sum = 0;

    println!("Processing file: {}", INPUT_FILE);

    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                sum += day_1::process_line(&l);
            }
        }
    }

    println!("{}", sum)
}
