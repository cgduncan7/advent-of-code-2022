use std::{cmp::Ordering, str::Lines};
use harness;

enum Result {
    Loss,
    Draw,
    Win
}

impl Result {
    fn from_str(str: &str) -> Result {
        match str {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            &_ => panic!("Unexpected string"),
        }
    }

    fn get_value(&self) -> i32 {
        match self {
            Result::Loss => 0,
            Result::Draw => 3,
            Result::Win => 6,
        }
    }
}

#[derive(Eq, Clone)]
enum Moveset {
    Rock,
    Paper,
    Scissors,
}

impl PartialEq for Moveset {
    fn eq(&self, other: &Self) -> bool {
        self.get_value().eq(&other.get_value())
    }
}

impl PartialOrd for Moveset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        let self_score = self.get_value();
        let other_score = other.get_value();
        let diff_scores = self_score - other_score;

        if diff_scores == -2 || diff_scores == 1 {
            return Some(Ordering::Greater);
        }

        if diff_scores == -1 || diff_scores == 2 {
            return Some(Ordering::Less);
        }

        None
    }
}

impl Ord for Moveset {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(x) => x,
            None => Ordering::Equal,
        }
    }
}

impl Moveset {
    fn get_move_from_result(&self, result: &Result) -> Moveset {
        match result {
            Result::Draw => self.clone(),
            Result::Loss => {
                match self {
                    Moveset::Rock => Moveset::Scissors,
                    Moveset::Paper => Moveset::Rock,
                    Moveset::Scissors => Moveset::Paper,
                }
            },
            Result::Win => {
                match self {
                    Moveset::Rock => Moveset::Paper,
                    Moveset::Paper => Moveset::Scissors,
                    Moveset::Scissors => Moveset::Rock,
                }
            }
        }
    }

    fn get_value(&self) -> i32 {
        match self {
            Moveset::Rock => 1,
            Moveset::Paper => 2,
            Moveset::Scissors => 3,
        }
    }

    fn get_result(&self, opponent: &Moveset) -> Result {
        match self.cmp(opponent) {
            Ordering::Less => Result::Loss,
            Ordering::Equal => Result::Draw,
            Ordering::Greater => Result::Win,
        }
    }

    fn get_total_score(&self, opponent: &Moveset) -> i32 {
        let mut total_score = self.get_value();
        total_score += self.get_result(opponent).get_value();
        total_score
    }
}

enum EnemyMove {
    A,
    B,
    C,
}

impl EnemyMove {
    fn as_move(&self) -> Moveset {
        match self {
            EnemyMove::A => Moveset::Rock,
            EnemyMove::B => Moveset::Paper,
            EnemyMove::C => Moveset::Scissors,
        }
    }

    fn from_str(str: &str) -> EnemyMove {
        match str {
            "A" => EnemyMove::A,
            "B" => EnemyMove::B,
            "C" => EnemyMove::C,
            &_ => panic!("Non-matching string"),
        }
    }
}

enum MyMove {
    X,
    Y,
    Z,
}

impl MyMove {
    fn as_move(&self) -> Moveset {
        match self {
            MyMove::X => Moveset::Rock,
            MyMove::Y => Moveset::Paper,
            MyMove::Z => Moveset::Scissors,
        }
    }

    fn from_str(str: &str) -> MyMove {
        match str {
            "X" => MyMove::X,
            "Y" => MyMove::Y,
            "Z" => MyMove::Z,
            &_ => panic!("Non-matching string"),
        }
    }
}

fn part1(lines: &mut Lines) -> i32 {
    let mut score: i32 = 0;
    for line in lines {
        let moves: Vec<_> = line.split(' ').collect();
        let raw_enemy_move = match moves.get(0) {
            Some(m) => *m,
            None => panic!("Failed to get move"),
        };
        let raw_my_move = match moves.get(1) {
            Some(m) => *m,
            None => panic!("Failed to get move"),
        };
        let enemy_move = EnemyMove::from_str(raw_enemy_move).as_move();
        let my_move = MyMove::from_str(raw_my_move).as_move();
        let move_score = my_move.get_total_score(&enemy_move);
        score += move_score;
    }

    score
}

fn part2(lines: &mut Lines) -> i32 {
    let mut score: i32 = 0;
    for line in lines {
        let moves: Vec<_> = line.split(' ').collect();
        let raw_enemy_move = match moves.get(0) {
            Some(m) => *m,
            None => panic!("Failed to get move"),
        };
        let raw_result = match moves.get(1) {
            Some(m) => *m,
            None => panic!("Failed to get move"),
        };
        let enemy_move = EnemyMove::from_str(raw_enemy_move).as_move();
        let result: Result = Result::from_str(raw_result);
        let my_move = enemy_move.get_move_from_result(&result);
        let move_score = my_move.get_total_score(&enemy_move);
        score += move_score;
    }

    score
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./data.txt", &part2);
}
