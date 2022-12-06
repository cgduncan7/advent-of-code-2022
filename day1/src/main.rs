use std::path::Path;
use harness;

fn part1(path: &Path) -> i32 {
    let input = match harness::get_input(path) {
        Ok(i) => i,
        Err(_) => panic!("Failed to get input"),
    };
    
    let lines = input.lines();

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

fn part2(path: &Path) -> i32 {
    let input = match harness::get_input(path) {
        Ok(i) => i,
        Err(_) => panic!("Failed to get input"),
    };
    
    let lines = input.lines();

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
    harness::time_function(Path::new("example.txt"), &part1);
    harness::time_function(Path::new("data.txt"), &part1);
    harness::time_function(Path::new("data.txt"), &part2);
}
