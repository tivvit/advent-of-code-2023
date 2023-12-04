use std::fs::File;
use std::io;
use std::path::Path;
use std::io::BufRead;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn process_line(line: &str) -> u32 {
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

    return format!("{}{}", first, last).parse().unwrap_or(0);
}
