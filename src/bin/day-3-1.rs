mod utils;

const INPUT_FILE: &'static str = "./input/day-3-1.in";
// const INPUT_FILE: &'static str = "./input/day-3-1-small.in";
// const INPUT_FILE: &'static str = "./input/day-3-1-small-2.in";

fn is_alpha(array: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    if r >= array.len() || c >= array[0].len() {
        return false;
    }
    let val = array[r][c];
    if val == '.' || val.is_numeric() {
        return false;
    }
    return true;
}

fn solve(array: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;

    for (r, a) in array.iter().enumerate() {
        let mut num: String = String::new();
        let mut cnt = 0;
        for (c, cr) in a.iter().enumerate() {
            if cr.is_numeric() {
                num.push(*cr);
                cnt += 1;
                continue;
            }
            if cnt > 0 {
                let mut processed = false;
                println!("{} {}", num, sum);

                for rd in vec![-1i32, 0, 1] {
                    if processed {
                        break;
                    }
                    for cd in (-1 * (cnt + 1))..1i32 {
                        if is_alpha(array, usize::try_from(r as i32 + rd).unwrap_or(0), usize::try_from(c as i32 + cd).unwrap_or(0)) {
                            sum += num.parse::<i64>().unwrap();
                            println!("added {}", sum);
                            processed = true;
                            break;
                        }
                    }
                }
                num = String::new();
                cnt = 0;
            }
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
