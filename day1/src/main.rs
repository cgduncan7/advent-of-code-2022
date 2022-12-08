use std::str::Lines;
use harness;

fn part1(lines: &mut Lines) -> i32 {
    let mut current = 0;
    let mut max = std::i32::MIN;
    for line in lines {
        if line.len() == 0 {
            max = max.max(current);
            current = 0;
        } else {
            current += match i32::from_str_radix(line, 10) {
                Ok(n) => n,
                Err(_) => panic!("Failed to parse {} to i32", line),
            };
        }
    }

    max
}

fn part2(lines: &mut Lines) -> i32 {
    let mut current = 0;
    let mut top_three = [std::i32::MIN, std::i32::MIN, std::i32::MIN];
    for line in lines {
        if line.len() == 0 {
            top_three[0] = top_three[0].max(current);
            top_three.sort();
            current = 0;
        } else {
            current += match i32::from_str_radix(line, 10) {
                Ok(n) => n,
                Err(_) => panic!("Failed to parse {} to i32", line),
            };
        }
    }

    let tti = top_three.iter();
    let mut total = 0;
    for v in tti {
        total += v;
    }
    total
}

fn main() {
    harness::time_function("example.txt", &part1);
    harness::time_function("data.txt", &part1);
    harness::time_function("data.txt", &part2);
}
