mod day_1;
mod utils;

// const INPUT_FILE: &'static str = "./input/day-1-2-small.in";
const INPUT_FILE: &'static str = "./input/day-1-2.in";

fn replace_string_digits(line: &str) -> String {
    // TODO make this const static
    let str_digits: Vec<(&str, &str)> = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut line_with_digits = String::from("");
    let mut i = 0;
    while i < line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_numeric() {
            line_with_digits.push_str(c.to_string().as_str());
        }

        for d in &str_digits {
            if line[i..].starts_with(d.0) {
                line_with_digits.push_str(d.1.to_string().as_str());
                break;
            }
        }
        i += 1;
    }
    return line_with_digits;
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
                sum += day_1::process_line(&replace_string_digits(&l));
            }
        }
    }

    println!("{}", sum)
}
