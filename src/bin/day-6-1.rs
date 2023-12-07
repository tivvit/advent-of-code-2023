mod utils;

const INPUT_FILE: &'static str = "./input/day-6-1.in";


fn fill_data_from_input(data: &mut Vec<i32>, input: &str) {
    for i in input.split(' ') {
        if i.len() == 0 {
            continue;
        }
        data.push(i.parse().unwrap());
    }
}

fn ways_to_win(times: &Vec<i32>, distances: &Vec<i32>) -> i32 {
    let mut answer = 1;
    let mut ways_to_win;

    for (t, d) in times.iter().zip(distances.iter()) {
        ways_to_win = 0;
        for tw in 0..*t {
            let dist = (t - tw) * tw;
            println!("{t} {d} {tw} {dist}");
            if dist > *d {
                ways_to_win += 1;
            }
        }
        answer *= ways_to_win;
    }

    return answer;
}

fn main() {
    println!("Processing file: {}", INPUT_FILE);

    let mut times = vec![];
    let mut distances = vec![];

    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::read_lines(INPUT_FILE) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);

                let mut line_in = l.split(':');
                match line_in.next().unwrap() {
                    "Time" => fill_data_from_input(&mut times, line_in.next().unwrap()),
                    "Distance" => fill_data_from_input(&mut distances, line_in.next().unwrap()),
                    x => println!("Unknown input: {}", x)
                }
            }
        }
    }

    println!("{}", ways_to_win(&times, &distances))
}
