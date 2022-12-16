use std::{str::Lines, collections::HashMap};
// use std::thread::sleep;
// use std::time;
use harness;

#[derive(Clone, Debug)]
struct Sand {
    location: (i32, i32),
}

#[derive(Clone, Debug)]
struct RockStructure {
    vertices: Vec<(i32, i32)>
}

impl RockStructure {
    fn new(s: &str) -> RockStructure {
        let mut vertices = Vec::new();
        for v in s.split(" -> ") {
            let nums = v.split(",")
                .map(|x| i32::from_str_radix(x, 10).unwrap())
                .collect::<Vec<i32>>();
            vertices.push((nums[0], nums[1]));   
        }
        RockStructure { vertices }
    }
}


#[derive(Debug)]
enum SpaceMapItem {
    Sand,
    Rock,
}

#[derive(PartialEq)]
enum SandState {
    Resting,
    Falling,
    IntoTheVoid,
    BlockingSpawn,
}

#[derive(Debug)]
struct Environment {
    step_count: u128,
    bounding_box: ((i32, i32), (i32, i32)),
    space_map: HashMap<(i32, i32), SpaceMapItem>,
    sand_spawn_loc: (i32, i32),
    sand: Vec<Sand>,
}

impl Environment {
    fn new(rock_structures: Vec<RockStructure>, sand_spawn_loc: (i32, i32), with_floor: bool) -> Environment {
        let mut space_map = HashMap::new();
        
        let mut min_x = sand_spawn_loc.0;
        let mut min_y = sand_spawn_loc.1;
        let mut max_x = sand_spawn_loc.0;
        let mut max_y = sand_spawn_loc.1;

        for rs in rock_structures.clone() {
            let mut prev_vertex: Option<(i32, i32)> = None;
            for vert in rs.vertices {
                if prev_vertex.is_none() {
                    prev_vertex = Some(vert);
                    if vert.0 < min_x {
                        min_x = vert.0;
                    }

                    if vert.0 > max_x {
                        max_x = vert.0;
                    }
                    
                    if vert.1 < min_y {
                        min_y = vert.1;
                    }

                    if vert.1 > max_y {
                        max_y = vert.1;
                    }
                    continue;
                }

                let u_prev_vertex = prev_vertex.unwrap();
                let px = u_prev_vertex.0;
                let py = u_prev_vertex.1;

                let x = vert.0;
                let y = vert.1;

                if vert.0 < min_x {
                    min_x = vert.0;
                }

                if vert.0 > max_x {
                    max_x = vert.0;
                }
                
                if vert.1 < min_y {
                    min_y = vert.1;
                }

                if vert.1 > max_y {
                    max_y = vert.1;
                }
                
                let x_diff = px.abs_diff(x);

                if x_diff == 0 {
                    let max = py.max(y);
                    let min = py.min(y);
                    for new_y in min..max+1 {
                        let loc = (x, new_y);
                        space_map.insert(loc, SpaceMapItem::Rock);
                    }
                } else {
                    let max = px.max(x);
                    let min = px.min(x);
                    for new_x in min..max+1 {
                        let loc = (new_x, y);
                        space_map.insert(loc, SpaceMapItem::Rock);
                    }
                }

                prev_vertex = Some(vert);
            }
        }

        if with_floor {
            let height = 2*max_y-1;
            for x in min_x-height..max_x+height {
                let loc = (x, max_y + 2);
                space_map.insert(loc, SpaceMapItem::Rock);
            }
            max_y += 2;
        }

        let sand_vec: Vec<Sand> = Vec::new();

        Environment {
            step_count: 0,
            space_map,
            sand: sand_vec,
            bounding_box: ((min_x, min_y), (max_x, max_y)),
            sand_spawn_loc,
        }
    }

    fn add_sand(&mut self) {
        let new_sand =  Sand { location: self.sand_spawn_loc };
        self.sand.push(new_sand);
        self.space_map.insert(self.sand_spawn_loc, SpaceMapItem::Sand);
    }

    fn update(&mut self) -> SandState {
        let mut moving_sand = self.sand.pop().unwrap();
        self.space_map.remove(&moving_sand.location);

        let mut sand_state = SandState::Resting;

        // check underneath
        let under_sand_loc = (moving_sand.location.0, moving_sand.location.1+1);
        let under_left_sand_loc = (under_sand_loc.0-1, under_sand_loc.1);
        let under_right_sand_loc = (under_sand_loc.0+1, under_sand_loc.1);

        if !self.space_map.contains_key(&under_sand_loc) {
            moving_sand.location = under_sand_loc;
            sand_state = SandState::Falling;
        } else if !self.space_map.contains_key(&under_left_sand_loc) {
            moving_sand.location = under_left_sand_loc;
            sand_state = SandState::Falling;
        } else if !self.space_map.contains_key(&under_right_sand_loc) {
            moving_sand.location = under_right_sand_loc;
            sand_state = SandState::Falling;
        }

        if moving_sand.location.1 > self.bounding_box.1.1 {
            sand_state = SandState::IntoTheVoid;
        }

        if moving_sand.location == self.sand_spawn_loc {
            sand_state = SandState::BlockingSpawn;
        }

        self.step_count += 1;
        self.space_map.insert(moving_sand.location, SpaceMapItem::Sand);
        self.sand.push(moving_sand);
        
        sand_state
    }
}

impl ToString for Environment {
    fn to_string(&self) -> String {

        let extra_bounds = ((-5,-1),(5,5));

        let mut str = String::new();
        str.push('\n');

        let min_loc = self.bounding_box.0;
        let max_loc = self.bounding_box.1;

        let y_range = min_loc.1+extra_bounds.0.1..max_loc.1+extra_bounds.1.1+1;
        for y in y_range {
            let x_range = min_loc.0+extra_bounds.0.0..max_loc.0+extra_bounds.1.0+1;
            for x in x_range {
                if (x,y) == self.sand_spawn_loc {
                    str.push('+');
                    continue;
                }

                let c = match self.space_map.get(&(x, y)) {
                    Some(i) => {
                        match i {
                            SpaceMapItem::Rock => '#',
                            SpaceMapItem::Sand => 'O',
                        }
                    },
                    None => '.',
                };

                str.push(c);
            }
            str.push('\n');
        }
        str.push('\n');
        str
    }
}

fn part1(lines: &mut Lines) -> usize {
    let mut rock_structures = Vec::new();
    for line in lines {
        rock_structures.push(RockStructure::new(line));
    }
    let sand_start_loc = (500, 0);
    let mut env = Environment::new(rock_structures, sand_start_loc, false);
    
    loop {
        env.add_sand();
        // println!("{}", env.to_string());
        
        loop {
            let is_resting = env.update();
            // println!("{}", env.to_string());
            // sleep(time::Duration::from_millis(1));
            match is_resting {
                SandState::Resting => break,
                SandState::IntoTheVoid => return env.sand.len() - 1,
                _ => continue,
            }
        }
    }
}

fn part2(lines: &mut Lines) -> usize {
    let mut rock_structures = Vec::new();
    for line in lines {
        rock_structures.push(RockStructure::new(line));
    }
    let sand_start_loc = (500, 0);
    let mut env = Environment::new(rock_structures, sand_start_loc, true);
    
    loop {
        env.add_sand();
        // println!("{}", env.to_string());
        
        loop {
            let is_resting = env.update();
            // println!("{}", env.to_string());
            // sleep(time::Duration::from_millis(10));
            match is_resting {
                SandState::Resting => break,
                SandState::BlockingSpawn => return env.sand.len(),
                SandState::IntoTheVoid => panic!("Shouldn't be a void!"),
                _ => continue,
            }
        }
    }
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}
