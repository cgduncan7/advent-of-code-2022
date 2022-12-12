use std::hash::Hash;
use std::str::Lines;
use std::collections::{HashMap, HashSet};
use harness;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Location {
    FromTop(usize),
    FromBottom(usize),
    FromLeft(usize),
    FromRight(usize),
}

impl Location {
    fn from_direction(dir: &Direction, dim: usize) -> Location {
        match dir {
            Direction::Top => Location::FromTop(dim),
            Direction::Bottom => Location::FromBottom(dim),
            Direction::Left => Location::FromLeft(dim),
            Direction::Right => Location::FromRight(dim),
        }
    }
}

struct TreePatch {
    tree_heights: Vec<usize>,
    dimension: usize,
    scenic_score_memo: HashMap<Location, usize>,
}

impl TreePatch {
    fn new(tree_heights: Vec<usize>) -> TreePatch {
        let dimension = (tree_heights.len() as f64).sqrt() as usize;

        TreePatch {
            tree_heights: tree_heights,
            dimension: dimension,
            scenic_score_memo: HashMap::new(),
        }
    }

    fn pass(&mut self, dir: Direction) -> Vec<usize> {
        let mut visible_indices: Vec<usize> = Vec::new();
        let mut max_heights: HashMap<Location, usize> = HashMap::new();

        let inner_dim = self.dimension - 2;
        let start_index = self.dimension;

        let indices_in_order = match dir {
            Direction::Top => {
                let mut v: Vec<usize> = Vec::new();
                for i in 0..inner_dim+1 {
                    v.append(
                        &mut ((start_index * i + 1)..(start_index * i + inner_dim + 1))
                            .collect::<Vec<usize>>()
                    );
                }
                v
            },
            Direction::Bottom => {
                let mut v: Vec<usize> = Vec::new();
                for i in 0..inner_dim+1 {
                    v.append(
                        &mut ((start_index * ((inner_dim + 1) - i) + 1)..(start_index * ((inner_dim + 1) - i) + inner_dim + 1))
                        .collect::<Vec<usize>>()
                    );
                }
                v
            },
            Direction::Left => {
                let mut v: Vec<usize> = Vec::new();
                for i in 0..inner_dim+1 {
                    v.append(
                        &mut ((start_index + i)..((start_index + i + (self.dimension * inner_dim)))).step_by(self.dimension)
                        .collect::<Vec<usize>>()
                    );
                }
                v
            },
            Direction::Right => {
                let mut v: Vec<usize> = Vec::new();
                for i in 0..inner_dim+1 {
                    v.append(
                        &mut ((start_index + (inner_dim + 1 - i))..(((start_index + (inner_dim + 1 - i)) + (self.dimension * inner_dim)))).step_by(self.dimension)
                            .collect::<Vec<usize>>()
                    );
                }
                v
            }
        };

        for i in indices_in_order {
            let dir_dim = match dir {
                Direction::Top => i % self.dimension,
                Direction::Bottom => i % self.dimension,
                Direction::Left => i / self.dimension,
                Direction::Right => i / self.dimension,
            };
            let loc = Location::from_direction(&dir, dir_dim);
            match max_heights.get(&loc) {
                Some(h) => {
                    if &self.tree_heights[i] > h {
                        max_heights.insert(loc, self.tree_heights[i]);
                        visible_indices.push(i);
                    }
                },
                None => {
                    // is an edge and already counted
                    max_heights.insert(loc, self.tree_heights[i]);
                    visible_indices.push(i);
                }
            }
        }

        visible_indices
    }

    fn get_visible_trees(&mut self) -> HashSet<usize> {
        let mut visible_tree_indices: HashSet<usize> = HashSet::new();
        self.pass(Direction::Top).iter().for_each(|i| {
            visible_tree_indices.insert(*i);
        });
        self.pass(Direction::Bottom).iter().for_each(|i| {
            visible_tree_indices.insert(*i);
        });
        self.pass(Direction::Left).iter().for_each(|i| {
            visible_tree_indices.insert(*i);
        });
        self.pass(Direction::Right).iter().for_each(|i| {
            visible_tree_indices.insert(*i);
        });
        
        visible_tree_indices
    }

    fn get_scenic_score(&mut self, index: usize) -> usize {
        self.get_scenic_score_from_dir(index, Direction::Top) *
        self.get_scenic_score_from_dir(index, Direction::Bottom) *
        self.get_scenic_score_from_dir(index, Direction::Left) * 
        self.get_scenic_score_from_dir(index, Direction::Right)
    }

    fn get_index_from_dir(&self, index: usize, displacement: usize, dir: Direction) -> Option<usize> {
        match dir {
            Direction::Top => index.checked_sub(self.dimension * displacement),
            Direction::Bottom => Some(index + (self.dimension * displacement)),
            Direction::Left => {
                if index % self.dimension == 0 {
                    None
                } else {
                    index.checked_sub(displacement)
                }
            },
            Direction::Right => {
                if index % self.dimension == self.dimension - 1 {
                    None
                } else {
                    Some(index + displacement)
                }
            },
        }
    }

    fn get_scenic_score_from_dir(&mut self, index: usize, dir: Direction) -> usize {
        if let Some(v) = self.scenic_score_memo.get(&Location::from_direction(&dir, index)) {
            return *v;
        };

        let height = self.tree_heights[index];

        let mut comparison_index = self.get_index_from_dir(index, 1, dir);
        let mut scenic_score = 0;
        let mut initial_add = false;
        loop {
            let comparison_height = match comparison_index {
                Some(i) => self.tree_heights.get(i),
                None => None,
            };
    
            let cnt: bool;
            let mut to_add = 0;
            to_add += match comparison_height {
                Some(a) => {
                    if comparison_index.is_none() {
                        cnt = false;
                        0
                    } else {
                        initial_add = true;
                        if &height > a {
                            let disp = self.get_scenic_score_from_dir(comparison_index.unwrap(), dir);
                            comparison_index = self.get_index_from_dir(comparison_index.unwrap(), disp, dir);
                            cnt = true;
                            disp
                        } else {
                            cnt = false;
                            0
                        }
                    }

                },
                None => {
                    cnt = false;
                    0
                },
            };
            scenic_score += to_add;
            if !cnt || to_add == 0 {
                break;
            }
        }

        scenic_score += if initial_add { 1 } else { 0 };
        self.scenic_score_memo.insert(Location::from_direction(&dir, index), scenic_score);
        scenic_score
    }

    fn get_highest_scenic_score(&mut self) -> (usize, usize) {
        let mut highest_scenic_score = 0;
        let mut index = 0;
        for i in 0..self.tree_heights.len() {
            let scenic_score = self.get_scenic_score(i);
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
                index = i;
            }
        }
        (index, highest_scenic_score)
    }
}

fn part1(lines: &mut Lines) -> usize {
    let tree_heights =
        lines.map(|l| l.to_string()).collect::<Vec<String>>()
        .join("")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut tree_patch = TreePatch::new(tree_heights.clone());
    let visible_tree_indices = tree_patch.get_visible_trees();

    // uncomment to visualize; imagine the corners are there too
    // for (index, height) in tree_heights.iter().enumerate() {
    //     if index % tree_patch.dimension == 0 {
    //         println!();
    //     }
    //     if visible_tree_indices.contains(&index) {
    //         print!("{:?}", height);
    //     } else {
    //         print!(" ");
    //     }
    // }

    visible_tree_indices.len() + 4
}

fn part2(lines: &mut Lines) -> usize {
    let tree_heights =
        lines.map(|l| l.to_string()).collect::<Vec<String>>()
        .join("")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut tree_patch = TreePatch::new(tree_heights.clone());
    let (_index, score) = tree_patch.get_highest_scenic_score();

    // uncomment to visualize
    // let top_to_show = 0..tree_patch.get_scenic_score_from_dir(_index, Direction::Top);
    // let left_to_show = 0..tree_patch.get_scenic_score_from_dir(_index, Direction::Left);
    // let right_to_show = 0..tree_patch.get_scenic_score_from_dir(_index, Direction::Right);
    // let bottom_to_show = 0..tree_patch.get_scenic_score_from_dir(_index, Direction::Bottom);

    // let mut indices_to_show: Vec<usize> = Vec::new();

    // for i in top_to_show {
    //     indices_to_show.push(tree_patch.get_index_from_dir(_index, i+1, Direction::Top).unwrap());
    // }
    // for i in left_to_show {
    //     indices_to_show.push(tree_patch.get_index_from_dir(_index, i+1, Direction::Left).unwrap());
    // }
    // for i in right_to_show {
    //     indices_to_show.push(tree_patch.get_index_from_dir(_index, i+1, Direction::Right).unwrap());
    // }
    // for i in bottom_to_show {
    //     indices_to_show.push(tree_patch.get_index_from_dir(_index, i+1, Direction::Bottom).unwrap());
    // }

    // for (i, height) in tree_heights.iter().enumerate() {
    //     if i % tree_patch.dimension == 0 {
    //         println!();
    //     }
    //     if indices_to_show.contains(&i) {
    //         print!("{}", height);
    //     } else if i == _index {
    //         print!("{}", height);
    //     } else {
    //         print!("_");
    //     }
    // }

    score
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}
