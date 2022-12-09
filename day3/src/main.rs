use std::collections::{HashSet, HashMap};
use std::str::Lines;
use harness;

fn char_to_pval (c: char) -> u32 {
    let value_str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let val = match value_str.find(c) {
        Some(v) => v + 1,
        None => panic!("Something wrong with char_to_pval"),
    };

    val as u32
}

fn part1(lines: &mut Lines) -> u32 {
    let mut total_priority = 0;
    for line in lines {
        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
        
        let mut unique_items = HashSet::new();
        let mut unique_duplicates = HashSet::new();
        for item in first_compartment.chars() {
            unique_items.insert(item);
        }

        for item in second_compartment.chars() {
            if unique_items.contains(&item) && !unique_duplicates.contains(&item) {
                unique_duplicates.insert(item);
                total_priority += char_to_pval(item);
            }
        }
    }
    
    total_priority
}

fn part2(lines: &mut Lines) -> u32 {
    let mut total_priority = 0;

    while let Some(first_elf) = lines.next() {
        let second_elf = lines.next().unwrap();
        let third_elf = lines.next().unwrap();
        
        let mut unique_duplicates = HashMap::new();

        for item in first_elf.chars() {
            unique_duplicates.insert(item, 1);
        }

        for item in second_elf.chars() {
            if unique_duplicates.contains_key(&item) {
                unique_duplicates.insert(item, 2);
            }
        }

        for item in third_elf.chars() {
            if unique_duplicates.contains_key(&item) && unique_duplicates.get(&item) == Some(&2) {
                total_priority += char_to_pval(item);
                break;
            }
        }
    }
    
    total_priority
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./data.txt", &part2);
}
