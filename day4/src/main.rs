use std::str::Lines;
use harness;

struct Assignment {
    lower_bound: u32,
    upper_bound: u32,
}

impl Assignment {
    fn from_str(str: &str) -> Assignment {
        let parts: Vec<&str> = str.split("-").collect();
        let lower_bound = match u32::from_str_radix(parts[0], 10) {
            Ok(v) => v,
            Err(_) => panic!("Error parsing lower bound"),
        };
        let upper_bound = match u32::from_str_radix(parts[1], 10) {
            Ok(v) => v,
            Err(_) => panic!("Error parsing lower bound"),
        };

        Assignment { lower_bound: lower_bound, upper_bound: upper_bound }
    }

    fn is_within(&self, other: &Assignment) -> bool {
        self.lower_bound >= other.lower_bound && self.upper_bound <= other.upper_bound
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        if self.lower_bound > other.upper_bound || self.upper_bound < other.lower_bound {
            return false;
        }

        true
    }
}

fn part1(lines: &mut Lines) -> u32 {
    let mut total_contained = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        let left_assignment = Assignment::from_str(parts[0]);
        let right_assignment = Assignment::from_str(parts[1]);

        if left_assignment.is_within(&right_assignment) || right_assignment.is_within(&left_assignment) {
            total_contained += 1;
        }
    }

    total_contained
}

fn part2(lines: &mut Lines) -> u32 {
    let mut total_contained = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        let left_assignment = Assignment::from_str(parts[0]);
        let right_assignment = Assignment::from_str(parts[1]);

        if left_assignment.overlaps(&right_assignment) {
            total_contained += 1;
        }
    }

    total_contained
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}