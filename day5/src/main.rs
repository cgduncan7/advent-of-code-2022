use std::str::Lines;
use harness;

#[derive(Debug)]
struct Crate {
    letter: char
}

#[derive(Debug)]
struct CrateStack {
    crates: Vec<Crate>
}

impl CrateStack {
    fn take(&mut self, number: usize, is_crate_mover_9001: bool) -> Vec<Crate> {
        let mut x = self.crates.split_off(self.crates.len() - number);
        if !is_crate_mover_9001 {
            x.reverse();
        }
        x
    }

    fn add(&mut self, crates: &mut Vec<Crate>) {
        self.crates.append(crates);
    }
}

fn part1(lines: &mut Lines) -> String {
    let mut mode = 0;
    let mut crate_stacks: Vec<CrateStack> = Vec::new();
    for line in lines {
        if line.contains(char::is_numeric) {
            mode = 1;
        }

        if mode == 0 {
            let mut chars = line.chars();
            let mut index = 0;
            loop {
                let first = match chars.next() {
                    Some(c) => c,
                    None => break,
                };

                let cs = match crate_stacks.get_mut(index) {
                    Some(c) => c,
                    None => {
                        let s = CrateStack { crates: Vec::new() };
                        crate_stacks.insert(index, s);
                        crate_stacks.get_mut(index).unwrap()
                    }
                };
                
                if first == '[' {
                    let crate_letter = match chars.next() {
                        Some(c) => c,
                        None => panic!("Failed to parse line"),
                    };
                    cs.crates.insert(0, Crate { letter: crate_letter });
                } else {
                    chars.next();
                }
                chars.next();
                chars.next();

                index += 1;
            }
        } else if mode == 1 {
            if line.starts_with('m') {
                let splits: Vec<&str> = line.split(' ').collect();
                let quantity = usize::from_str_radix(splits[1], 10).unwrap();
                let from = usize::from_str_radix(splits[3], 10).unwrap() - 1;
                let to = usize::from_str_radix(splits[5], 10).unwrap() - 1;
                
                let from_stack = match crate_stacks.get_mut(from) {
                    Some(cs) => cs,
                    None => panic!("Missing crate stack! {}", from),
                };
                let mut crane_crates = from_stack.take(quantity, false);

                let to_stack = match crate_stacks.get_mut(to) {
                    Some(cs) => cs,
                    None => panic!("Missing crate stack! {}", to),
                };

                to_stack.add(&mut crane_crates);
            }
        }
    }

    let mut result = String::new();
    for crate_stack in crate_stacks.iter() {
        if let Some(cr8t) = crate_stack.crates.last() {
            result.push(cr8t.letter);
        }
    }

    result
}

fn part2(lines: &mut Lines) -> String {
    let mut mode = 0;
    let mut crate_stacks: Vec<CrateStack> = Vec::new();
    for line in lines {
        if line.contains(char::is_numeric) {
            mode = 1;
        }

        if mode == 0 {
            let mut chars = line.chars();
            let mut index = 0;
            loop {
                let first = match chars.next() {
                    Some(c) => c,
                    None => break,
                };

                let cs = match crate_stacks.get_mut(index) {
                    Some(c) => c,
                    None => {
                        let s = CrateStack { crates: Vec::new() };
                        crate_stacks.insert(index, s);
                        crate_stacks.get_mut(index).unwrap()
                    }
                };
                
                if first == '[' {
                    let crate_letter = match chars.next() {
                        Some(c) => c,
                        None => panic!("Failed to parse line"),
                    };
                    cs.crates.insert(0, Crate { letter: crate_letter });
                } else {
                    chars.next();
                }
                chars.next();
                chars.next();

                index += 1;
            }
        } else if mode == 1 {
            if line.starts_with('m') {
                let splits: Vec<&str> = line.split(' ').collect();
                let quantity = usize::from_str_radix(splits[1], 10).unwrap();
                let from = usize::from_str_radix(splits[3], 10).unwrap() - 1;
                let to = usize::from_str_radix(splits[5], 10).unwrap() - 1;
                
                let from_stack = match crate_stacks.get_mut(from) {
                    Some(cs) => cs,
                    None => panic!("Missing crate stack! {}", from),
                };
                let mut crane_crates = from_stack.take(quantity, true);

                let to_stack = match crate_stacks.get_mut(to) {
                    Some(cs) => cs,
                    None => panic!("Missing crate stack! {}", to),
                };

                to_stack.add(&mut crane_crates);
            }
        }
    }

    let mut result = String::new();
    for crate_stack in crate_stacks.iter() {
        if let Some(cr8t) = crate_stack.crates.last() {
            result.push(cr8t.letter);
        }
    }

    result
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./data.txt", &part2);
}
