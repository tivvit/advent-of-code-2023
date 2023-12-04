use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INPUT_FILE: &'static str = "./input/day-1-1.in";

fn process_line(line: &str) -> u32 {
    let mut first: char = ' ';
    let mut last: char = ' ';

    for c in line.chars() {
        if c.is_ascii_digit() {
            if first == ' ' {
                first = c;
            }
            last = c;
        }
    }

    println!("{} -> {} {}", line, first, last);

    return format!("{}{}", first, last).parse().unwrap_or(0)
}

fn main() {
    let mut sum = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(INPUT_FILE) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                sum += process_line(&l);
            }
        }
    }

    println!("{}", sum)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
