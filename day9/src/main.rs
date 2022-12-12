use std::{str::Lines, collections::HashSet};
use harness;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(s: &str) -> Direction {
        match s {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            &_ => panic!("WHAT DIRECTION IS THAT"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn distance_to(&self, coord: Coordinates) -> Coordinates {
        Coordinates {
            x: coord.x - self.x,
            y: coord.y - self.y,
        }
    }

    fn update(&mut self, coord: Coordinates) {
        self.x += coord.x;
        self.y += coord.y;
    }

    fn max_mag(&self) -> i32 {
        let abs_x = self.x.abs();
        let abs_y = self.y.abs();
        if abs_x > abs_y {
            abs_x
        } else {
            abs_y
        }
    }
}

#[derive(Clone, Debug)]
struct Rope {
    knots: Vec<Coordinates>,
}

impl Rope {
    fn new(size: usize) -> Rope {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(Coordinates { x: 0, y: 0 });
        }
        Rope {
            knots: v,
        }
    }

    fn move_head(&mut self, dir: &Direction) {
        let c = match dir {
            Direction::Up => {
                Coordinates { x: 0, y: 1 }
            },
            Direction::Down => {
                Coordinates { x: 0, y: -1 }
            },
            Direction::Left => {
                Coordinates { x: -1, y: 0 }
            },
            Direction::Right => {
                Coordinates { x: 1, y: 0 }
            },
        };

        self.knots[0].update(c);
    }

    fn move_knot(&mut self, index: usize) {
        let previous_knot = self.knots[index-1];
        let dist = self.knots[index].distance_to(previous_knot);

        let update = match dist.max_mag() {
            0 | 1 => {
                Coordinates { x: 0, y: 0 }
            },
            2 => {
                let mut x_move = 0;
                if dist.x > 0 {
                    x_move = 1;
                } else if dist.x < 0 {
                    x_move = -1;
                }

                let mut y_move = 0;
                if dist.y > 0 {
                    y_move = 1;
                } else if dist.y < 0 {
                    y_move = -1;
                }

                Coordinates { x: x_move, y: y_move }
            },
            _ => {
                panic!("Tail is too far from head, something is wrong");
            },
        };

        self.knots[index].update(update);
    }

    fn motion(&mut self, dir: &Direction) {
        self.move_head(dir);
        for (index, _coords) in self.knots.clone().iter().enumerate().skip(1) {
            self.move_knot(index);
        }
    }
}

fn part1(lines: &mut Lines) -> usize {
    let mut rope = Rope::new(2);
    let mut unique_tails_coords: HashSet<Coordinates> = HashSet::new();
    for line in lines {
        let splits = line.split(" ").collect::<Vec<&str>>();
        let dir = splits[0];
        let amt = u32::from_str_radix(splits[1], 10).unwrap();
        (0..amt).for_each(|_| {
            rope.motion(&Direction::from(dir));
            unique_tails_coords.insert(*rope.knots.last().unwrap());
        });
    }

    unique_tails_coords.len()
}

fn part2(lines: &mut Lines) -> usize {
    let mut rope = Rope::new(10);
    let mut unique_tails_coords: HashSet<Coordinates> = HashSet::new();
    for line in lines {
        let splits = line.split(" ").collect::<Vec<&str>>();
        let dir = splits[0];
        let amt = u32::from_str_radix(splits[1], 10).unwrap();
        (0..amt).for_each(|_| {
            rope.motion(&Direction::from(dir));
            unique_tails_coords.insert(*rope.knots.last().unwrap());
        });
    }

    unique_tails_coords.len()
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}
