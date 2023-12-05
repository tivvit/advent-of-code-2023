mod utils;

const INPUT_FILE: &'static str = "./input/day-5-1.in";

fn parse_seeds(line: &str) -> Vec<u64>{
    let mut seeds = vec![];
    for s in line.strip_prefix("seeds: ").unwrap().split(' ') {
        seeds.push(s.parse().unwrap());
    }

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
