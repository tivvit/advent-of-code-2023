mod utils;

const INPUT_FILE: &'static str = "./input/day-5-2.in";

fn parse_seeds(line: &str) -> Vec<u64>{
    let mut seeds = vec![];
    let mut start = 0;
    let mut size: u64;
    let mut cnt = 0;

    // TODO intervals needs to be used

    for s in line.strip_prefix("seeds: ").unwrap().split(' ') {
        if cnt % 2 == 0 {
            size = s.parse().unwrap();
            for i in start..(start+size) {
                seeds.push(i);
            }
        } else {
            start = s.parse().unwrap();
        }
        cnt += 1;
    }

    println!("{:?}", seeds);

    return seeds;
}

fn main() {
    println!("Processing file: {}", INPUT_FILE);

    let mut seeds= vec![];
    let mut new_seeds = vec![];

    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);

                if new_seeds.len() == 0 {
                    new_seeds = parse_seeds(&l);
                    continue;
                }

                // empty line - start processing new line
                if l.len() == 0  {
                    seeds = new_seeds.clone();
                    continue
                }

                if l.contains(":") {
                    continue;
                }

                let mut range_input = l.split(' ');
                let dest: u64 = range_input.nth(0).unwrap().parse().unwrap_or(0);
                let src: u64 = range_input.nth(0).unwrap().parse().unwrap_or(0);
                let size: u64 = range_input.nth(0).unwrap().parse().unwrap_or(0);

                for (i, s) in seeds.iter().enumerate() {
                    if *s >= src && *s <= src + size {
                        new_seeds[i] = dest + (*s - src);
                    }
                }
            }
        }
    }

    println!();
    println!("{}", new_seeds.iter().min().unwrap())
}
