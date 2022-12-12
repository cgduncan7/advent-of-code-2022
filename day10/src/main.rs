use std::str::Lines;
use harness;

fn part1(lines: &mut Lines) -> i128 {
    let mut cycle_count = 0;
    let mut x_register: i128 = 1;
    let mut signal_strengths: Vec<i128> = Vec::new();
    let cycle_markers = vec![20, 60, 100, 140, 180, 220];
    
    for line in lines {
        if line.eq("noop") {
            cycle_count += 1;
            if cycle_markers.contains(&cycle_count) {
                signal_strengths.push(cycle_count * x_register);
            }
        } else if line.starts_with("addx") {
            let to_add = i128::from_str_radix(&line.replace("addx ", ""), 10).unwrap();
            cycle_count += 1;
            if cycle_markers.contains(&cycle_count) {
                signal_strengths.push(cycle_count * x_register);
            }
            cycle_count += 1;
            if cycle_markers.contains(&cycle_count) {
                signal_strengths.push(cycle_count * x_register);
            }
            x_register += to_add;
        }
    }
    
    signal_strengths.iter().sum()
}

fn part2(lines: &mut Lines) -> usize {
    let mut cycle_count = 0;
    let mut x_register: i128 = 1;
    
    for line in lines {
        if line.eq("noop") {
            if cycle_count % 40 == 0 {
                println!();
            }
            
            if cycle_count % 40 >= x_register - 1 && cycle_count % 40 <= x_register + 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycle_count += 1;
        } else if line.starts_with("addx") {
            let to_add = i128::from_str_radix(&line.replace("addx ", ""), 10).unwrap();
            if cycle_count % 40 == 0 {
                println!();
            }
            if cycle_count % 40 >= x_register - 1 && cycle_count % 40 <= x_register + 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycle_count += 1;
            if cycle_count % 40 == 0 {
                println!();
            }
            if cycle_count % 40 >= x_register - 1 && cycle_count % 40 <= x_register + 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycle_count += 1;
            x_register += to_add;
        }
    }
    
    0
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}
