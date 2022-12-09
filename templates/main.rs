use std::str::Lines;
use harness;

fn part1(lines: &mut Lines) -> usize {
    0
}

fn part2(lines: &mut Lines) -> usize {
    0
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}
