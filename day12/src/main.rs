use std::cell::RefCell;
use std::str::Lines;
use harness;

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Start,
    End,
    Generic,
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    location: (usize, usize),
    height: usize,
    node_type: NodeType,
    visited_at: Option<usize>
}

impl Node {
    fn new(row: usize, col: usize, height_char: char, node_type: NodeType) -> Node {
        let height = match node_type {
            NodeType::Start => 1,
            NodeType::End => 26,
            NodeType::Generic => Node::height_from_char(height_char),
        };

        Node {
            location: (row, col),
            height,
            node_type,
            visited_at: None,
        }
    }

    fn height_from_char(c: char) -> usize {
        let heightmap = "abcdefghijklmnopqrstuvwxyz";
        heightmap.find(c).unwrap() + 1
    }

    fn can_traverse_to(&self, other: &Node) -> bool {
        self.height + 1 >= other.height
    }
}

fn part1(lines: &mut Lines) -> usize {
    let nodes: RefCell<Vec<Node>> = RefCell::new(Vec::new());
    let rows = lines.clone().count();
    let mut cols = 0;

    let mut start_loc: (usize, usize) = (0, 0);

    for (row, line) in lines.enumerate() {
        if cols == 0 {
            cols = line.len();
        }

        for (col, height_char) in line.chars().enumerate() {
            let node_type = match height_char {
                'S' => {
                    start_loc = (row, col);
                    NodeType::Start
                },
                'E' => NodeType::End,
                _ => NodeType::Generic,
            };
            let n = Node::new(row, col, height_char, node_type);
            nodes.borrow_mut().push(n);
        }
    }

    let start_node_index = start_loc.0 * cols + start_loc.1;

    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((start_node_index, 0));

    let mut answer = usize::MAX;

    while !stack.is_empty() {
        let (current_node_index, depth) = stack.pop().unwrap();
        if depth > answer {
            continue;
        }
        let neighbor_indices: Vec<usize>;
        {
            {
                nodes.borrow_mut()[current_node_index].visited_at = Some(depth);
            }

            let current_node = &nodes.borrow()[current_node_index];

            let (current_row, current_col) = current_node.location;
            if current_node.node_type == NodeType::End {
                if depth < answer {
                    answer = depth;
                }
                continue;
            }

            // get possible next steps
            // let index = current_row * cols + current_col;
            let neighbor_locs = vec![
                (current_row.checked_sub(1), Some(current_col)), // top
                (Some(current_row + 1), Some(current_col)), // bottom
                (Some(current_row), current_col.checked_sub(1)), // left
                (Some(current_row), Some(current_col + 1)), // right
            ];

            // branch into new paths
            neighbor_indices = neighbor_locs.iter()
                .filter(|(r, c)| r.is_some() && c.is_some())
                .map(|(r, c)| (r.unwrap(), c.unwrap()))
                .filter(|(r, c)| r >= &0 && r < &rows && c >= &0 && c < &cols)
                .map(|(r, c)| r * cols + c)
                .filter(|i| {
                    let n = &nodes.borrow()[*i];
                    let less_depth = n.visited_at.is_none() || n.visited_at.unwrap() > (depth + 1);
                    current_node.can_traverse_to(n) && less_depth
                })
                .collect::<Vec<usize>>();
        }

        for ni in neighbor_indices {
            nodes.borrow_mut()[ni].visited_at = Some(depth);
            stack.push((ni, depth + 1))
        }
    }

    answer
}

fn part2(lines: &mut Lines) -> usize {
    let nodes: RefCell<Vec<Node>> = RefCell::new(Vec::new());
    let rows = lines.clone().count();
    let mut cols = 0;

    let mut start_locs: Vec<(usize, usize)> = Vec::new();

    for (row, line) in lines.enumerate() {
        if cols == 0 {
            cols = line.len();
        }

        for (col, height_char) in line.chars().enumerate() {
            let node_type = match height_char {
                'S' | 'a' => {
                    start_locs.push((row, col));
                    NodeType::Start
                },
                'E' => NodeType::End,
                _ => NodeType::Generic,
            };
            let n = Node::new(row, col, height_char, node_type);
            nodes.borrow_mut().push(n);
        }
    }

    
    let mut stack: Vec<(usize, usize)> = Vec::new();

    for start_loc in start_locs {
        let start_node_index = start_loc.0 * cols + start_loc.1;
        stack.push((start_node_index, 0));
    }

    let mut answer = usize::MAX;

    while !stack.is_empty() {
        let (current_node_index, depth) = stack.pop().unwrap();
        if depth > answer {
            continue;
        }
        let neighbor_indices: Vec<usize>;
        {
            {
                nodes.borrow_mut()[current_node_index].visited_at = Some(depth);
            }

            let current_node = &nodes.borrow()[current_node_index];

            let (current_row, current_col) = current_node.location;
            if current_node.node_type == NodeType::End {
                if depth < answer {
                    answer = depth;
                }
                continue;
            }

            // get possible next steps
            // let index = current_row * cols + current_col;
            let neighbor_locs = vec![
                (current_row.checked_sub(1), Some(current_col)), // top
                (Some(current_row + 1), Some(current_col)), // bottom
                (Some(current_row), current_col.checked_sub(1)), // left
                (Some(current_row), Some(current_col + 1)), // right
            ];

            // branch into new paths
            neighbor_indices = neighbor_locs.iter()
                .filter(|(r, c)| r.is_some() && c.is_some())
                .map(|(r, c)| (r.unwrap(), c.unwrap()))
                .filter(|(r, c)| r >= &0 && r < &rows && c >= &0 && c < &cols)
                .map(|(r, c)| r * cols + c)
                .filter(|i| {
                    let n = &nodes.borrow()[*i];
                    let less_depth = n.visited_at.is_none() || n.visited_at.unwrap() > (depth + 1);
                    current_node.can_traverse_to(n) && less_depth
                })
                .collect::<Vec<usize>>();
        }

        for ni in neighbor_indices {
            nodes.borrow_mut()[ni].visited_at = Some(depth);
            stack.push((ni, depth + 1))
        }
    }

    answer
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}
