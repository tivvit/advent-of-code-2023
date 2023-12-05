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
