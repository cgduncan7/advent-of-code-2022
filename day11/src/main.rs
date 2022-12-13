use std::{str::Lines, cell::RefCell, borrow::BorrowMut};
use harness;

fn add(lhs: usize, rhs: usize) -> usize {
    lhs + rhs
}

fn mul(lhs: usize, rhs: usize) -> usize {
    match lhs.checked_mul(rhs) {
        Some(m) => m,
        None => {
            (lhs % 10000) * (rhs % 10000)
        }
    }
}

struct Movement {
    target_index: usize,
    item: usize,
}

struct Monkey {
    items: RefCell<Vec<usize>>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    target_true_index: usize,
    target_false_index: usize,
    num_inspections: usize,
}

impl Monkey {
    fn new(items_str: &str, op_str: &str, test_str: &str, target_true_str: &str, target_false_str: &str) -> Monkey {
        Monkey {
            items: RefCell::new(Monkey::parse_items_str(items_str)),
            operation: Box::new(Monkey::parse_operations_str(op_str)),
            test: Box::new(Monkey::parse_test_str(test_str)),
            target_true_index: Monkey::parse_target_str(target_true_str),
            target_false_index: Monkey::parse_target_str(target_false_str),
            num_inspections: 0,
        }
    }

    fn parse_items_str(input: &str) -> Vec<usize> {
        let numbers = input
            .replace("  Starting items: ", "")
            .split(", ")
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect::<Vec<usize>>();
        numbers
    }

    fn parse_operations_str(input: &str) -> impl Fn(usize) -> usize {
        let trimmed_str = input
            .replace("  Operation: ", "");
        let eq_sides = trimmed_str
            .split(" = ")
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let eq = eq_sides[1]
            .split(" ")
            .map(|x| String::from(x))
            .collect::<Vec<String>>();

        move |old| {
            if eq[2].eq("old") {
                return old * old;
            }

            let rhs = usize::from_str_radix(&eq[2], 10).unwrap();
    
            match eq[1].as_str() {
                "*" => mul(old, rhs),
                "+" => add(old, rhs),
                &_ => panic!("Unknown operation"),
            }
        }
    }

    fn parse_test_str(input: &str) -> impl Fn(usize) -> bool {
        let trimmed_str = input
            .replace("  Test: divisible by ", "");
        let divisor = usize::from_str_radix(&trimmed_str, 10).unwrap();

        move |item| {
            item % divisor == 0
        }
    }

    fn parse_target_str(input: &str) -> usize {
        let trimmed_str = input
            .replace("    If true: throw to monkey ", "")
            .replace("    If false: throw to monkey ", "");
        let index = usize::from_str_radix(&trimmed_str, 10).unwrap();
        index
    }

    fn take_turn(&mut self, reduce_worry: bool) -> Option<Movement> {
        if self.items.borrow().len() == 0 {
            return None;
        }
        let current_item = self.items.borrow_mut().remove(0);
        let mut new_item = (self.operation)(current_item);
        if reduce_worry {
            new_item /= 3;
        }
        self.num_inspections += 1;
        let target_monkey_index: usize;
        if (self.test)(new_item) {
            target_monkey_index = self.target_true_index;
        } else {
            target_monkey_index = self.target_false_index;
        }
        
        Some(Movement { target_index: target_monkey_index, item: new_item })
    }

    fn take_turns(&mut self, reduce_worry: bool) -> Vec<Movement> {
        let mut movements = Vec::new();
        let mut cnt = true;
        while cnt {
            match self.take_turn(reduce_worry) {
                Some(m) => movements.push(m),
                None => cnt = false,
            };
        }

        movements
    }
}

fn part1(lines: &mut Lines) -> usize {
    let mut monkeys: Vec<Monkey> = Vec::new();
    loop {
        let mut it = lines.take(7);
        if it.next().is_none() {
            break;
        }
        let items = it.next().unwrap();
        let operation = it.next().unwrap();
        let test = it.next().unwrap();
        let target_true = it.next().unwrap();
        let target_false = it.next().unwrap();
        it.next();

        let m = Monkey::new(items, operation, test, target_true, target_false);
        monkeys.push(m);
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let m = monkeys.get_mut(i).unwrap();
            let movements = m.take_turns(true);
            for movement in movements.iter() {
                monkeys.get(movement.target_index).borrow_mut().unwrap().items.borrow_mut().push(movement.item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.num_inspections);
    monkeys.reverse();
    
    monkeys[0].num_inspections * monkeys[1].num_inspections
}

fn part2(lines: &mut Lines) -> usize {
    let mut monkeys: Vec<Monkey> = Vec::new();
    loop {
        let mut it = lines.take(7);
        if it.next().is_none() {
            break;
        }
        let items = it.next().unwrap();
        let operation = it.next().unwrap();
        let test = it.next().unwrap();
        let target_true = it.next().unwrap();
        let target_false = it.next().unwrap();
        it.next();

        let m = Monkey::new(items, operation, test, target_true, target_false);
        monkeys.push(m);
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let m = monkeys.get_mut(i).unwrap();
            let movements = m.take_turns(false);
            for movement in movements.iter() {
                monkeys.get(movement.target_index).borrow_mut().unwrap().items.borrow_mut().push(movement.item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.num_inspections);
    monkeys.reverse();
    
    monkeys[0].num_inspections * monkeys[1].num_inspections
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    // harness::time_function("./example.txt", &part2);
    // harness::time_function("./data.txt", &part2);
}
