use std::collections::HashSet;

mod utils;

const INPUT_FILE: &'static str = "./input/day-3-2.in";

fn expand_num(array: &Vec<Vec<char>>, r: usize, c: usize, used: &mut HashSet<(usize, usize)>) -> i64 {
    let mut tmp = String::from(array[r][c]);
    used.insert((r, c));
    // expand left
    for i in 1..=c {
        let p= c - i;
        if !array[r][p].is_numeric() {
            break;
        }
        used.insert((r, p));
        tmp = String::from(array[r][p]) + &tmp;
    }
    // expand right
    for i in c + 1 ..=array[0].len() {
        if !array[r][i].is_numeric() {
            break;
        }
        used.insert((r, i));
        tmp += &array[r][i].to_string();
    }
    return tmp.parse().unwrap();
}

fn check_num(array: &Vec<Vec<char>>, r: usize, c: usize, used: &mut HashSet<(usize, usize)>) -> Option<i64> {
    if r >= array.len() || c >= array[0].len() {
        return None;
    }
    if !array[r][c].is_numeric() {
        return None;
    }
    if used.contains(&(r, c)) {
        return None;
    }
    return Option::from(expand_num(array, r, c, used));
}

fn gear(array: &Vec<Vec<char>>, r: usize, c: usize) -> i64 {
    let mut used = HashSet::new();
    let mut nums = vec![];

    for (rp, cp) in vec![(r - 1, c - 1), (r - 1, c), (r - 1, c + 1), (r, c - 1), (r, c + 1), (r + 1, c - 1), (r + 1, c), (r + 1, c + 1)] {
        let n = check_num(array, rp, cp, &mut used);
        if n.is_some() {
            println!("{}", n.unwrap());
            nums.push(n.unwrap());
        }
    }
    println!();
    if nums.len() != 2 {
        return 0;
    }
    return nums[0] * nums[1];
}

fn solve(array: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;

    for r in 0..array.len() {
        for c in 0..array[0].len() {
            if array[r][c] != '*' {
                continue;
            }
            sum += gear(array, r, c);
        }
    }

    return sum;
}

fn main() {
    println!("Processing file: {}", INPUT_FILE);

    let mut input: Vec<Vec<char>> = vec![];

    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                // println!("{}", l);

                let mut li: Vec<char> = vec![];
                for c in l.chars() {
                    li.push(c);
                }
                // Add . to the end to ensure processing on the end of the line.
                li.push('.');
                input.push(li);
            }
        }
    }

    println!("{}", solve(&input));
}
