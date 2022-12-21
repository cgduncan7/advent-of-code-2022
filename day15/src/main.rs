use std::str::Lines;
use harness;

#[derive(Debug, Eq, Ord)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn mag(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.mag()== other.mag()
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.mag().partial_cmp(&other.mag())
    }
}

#[derive(Debug)]
struct Beacon {
    location: Location,
    distance: Location,
}

impl Beacon {
    fn new(location: Location, closest_beacon: Location) -> Beacon {
        let distance = Location { x: location.x - closest_beacon.x, y: location.y - closest_beacon.y };
        Beacon {
            location,
            distance,
        }
    }

    fn get_distance(&self, other: &Location) -> Location {
        Location { x: self.location.x - other.x, y: self.location.y - other.y }
    }
}

static mut CHOSEN_ROW: i32 = 10;
static mut MAX_DIM: i32 = 20;

fn part1(lines: &mut Lines) -> usize {
    let mut min_dims: (i32, i32) = (i32::MAX, i32::MAX);
    let mut max_dims: (i32, i32) = (0, 0);
    let mut beacons: Vec<Beacon> = Vec::new();
    let mut closest_beacons: Vec<Location> = Vec::new();
    for line in lines {
        let line = line.replace(|c| c != ':' && c != ',' && !char::is_numeric(c) && c != '-', "");
        let mut split = line.split(":");
        let loc_strs = split.next().unwrap().split(",").collect::<Vec<&str>>();
        let location = (i32::from_str_radix(loc_strs[0], 10).unwrap(), i32::from_str_radix(loc_strs[1], 10).unwrap());
        let other_loc_strs = split.next().unwrap().split(",").collect::<Vec<&str>>();
        let closest_beacon = (i32::from_str_radix(other_loc_strs[0], 10).unwrap(), i32::from_str_radix(other_loc_strs[1], 10).unwrap());
        let beacon = Beacon::new(
            Location { x: location.0, y: location.1 },
            Location { x: closest_beacon.0, y: closest_beacon.1 },
        );
        let closest_beacon_location = Location { x: closest_beacon.0, y: closest_beacon.1 };
        closest_beacons.push(closest_beacon_location);

        if beacon.location.x + beacon.distance.mag() > max_dims.0 {
            max_dims.0 = beacon.location.x + beacon.distance.mag();
        }

        if beacon.location.y + beacon.distance.mag() > max_dims.1 {
            max_dims.1 = beacon.location.y + beacon.distance.mag();
        }

        if beacon.location.x - beacon.distance.mag() < min_dims.0 {
            min_dims.0 = beacon.location.x - beacon.distance.mag();
        }

        if beacon.location.y - beacon.distance.mag() < min_dims.1 {
            min_dims.1 = beacon.location.y - beacon.distance.mag();
        }

        beacons.push(beacon);
    }
    
    let chosen_row = unsafe { CHOSEN_ROW };

    // get all beacons that do not have a "service zone" which overlaps the chosen row
    let mut relevant_beacons = Vec::new();
    for beacon in beacons.iter() {
        let other_loc = Location { x: beacon.location.x, y: chosen_row };
        if beacon.get_distance(&other_loc) <= beacon.distance {
            // should be included
            relevant_beacons.push(beacon.clone());
        }
    }


    let mut result = 0;
    for x in min_dims.0..max_dims.0+1 {
        let current_location = Location{ x, y: chosen_row };
        if closest_beacons.contains(&current_location) {
            continue;
        }
        for beacon in relevant_beacons.iter() {
            if beacon.get_distance(&current_location) <= beacon.distance { 
                result += 1;
                break;
            }
        }
    }

    result
}

fn part2(lines: &mut Lines) -> i64 {
    let mut min_dims: (i32, i32) = (i32::MAX, i32::MAX);
    let mut max_dims: (i32, i32) = (0, 0);
    let mut beacons: Vec<Beacon> = Vec::new();
    let mut closest_beacons: Vec<Location> = Vec::new();
    for line in lines {
        let line = line.replace(|c| c != ':' && c != ',' && !char::is_numeric(c) && c != '-', "");
        let mut split = line.split(":");
        let loc_strs = split.next().unwrap().split(",").collect::<Vec<&str>>();
        let location = (i32::from_str_radix(loc_strs[0], 10).unwrap(), i32::from_str_radix(loc_strs[1], 10).unwrap());
        let other_loc_strs = split.next().unwrap().split(",").collect::<Vec<&str>>();
        let closest_beacon = (i32::from_str_radix(other_loc_strs[0], 10).unwrap(), i32::from_str_radix(other_loc_strs[1], 10).unwrap());
        let beacon = Beacon::new(
            Location { x: location.0, y: location.1 },
            Location { x: closest_beacon.0, y: closest_beacon.1 },
        );
        let closest_beacon_location = Location { x: closest_beacon.0, y: closest_beacon.1 };
        closest_beacons.push(closest_beacon_location);

        if beacon.location.x + beacon.distance.mag() > max_dims.0 {
            max_dims.0 = beacon.location.x + beacon.distance.mag();
        }

        if beacon.location.y + beacon.distance.mag() > max_dims.1 {
            max_dims.1 = beacon.location.y + beacon.distance.mag();
        }

        if beacon.location.x - beacon.distance.mag() < min_dims.0 {
            min_dims.0 = beacon.location.x - beacon.distance.mag();
        }

        if beacon.location.y - beacon.distance.mag() < min_dims.1 {
            min_dims.1 = beacon.location.y - beacon.distance.mag();
        }

        beacons.push(beacon);
    }
    
    let max_dim = unsafe { MAX_DIM };


    let result: i64;
    let mut x = 0;
    let mut y = 0;
    loop {
        let current_location = Location{ x, y };
        if !closest_beacons.contains(&current_location) {
            let mut overlapped = false;
            for beacon in beacons.iter() {
                let dist = beacon.get_distance(&current_location);
                if dist <= beacon.distance {
                    let distance_to_move_by = beacon.distance.mag() - dist.mag();
                    if x + distance_to_move_by > max_dim {
                        // determine whether we can skip more rows or not by checking the maximum x-value covered by this beacon is at this y-value
                        // if that value is less than 0 and the max-x-value is greater than MAX_DIM, we can skip a number of rows equal to that minimum
                        let x_dist_remaining = (beacon.distance.mag() - (beacon.location.y - dist.y).abs()).abs();
                        let min_x = current_location.x - x_dist_remaining;
                        let max_x = current_location.x + x_dist_remaining;
                        if min_x <= 0 && max_x >= 0 {
                            x = 0;
                            y += min_x.abs().min(max_x);
                        } else {
                            x = 0;
                            y += 1;
                        }
                    } else {
                        x += distance_to_move_by + 1;
                    }
                    overlapped = true;
                    break;
                }
            }

            if !overlapped {
                result = (current_location.x as i64) * 4000000 + (current_location.y as i64);
                break;
            }
        } else {
            x += 1;
        }

        if x > max_dim {
            x = 0;
            y += 1;
        }
    }

    result
}

fn main() {
    unsafe { CHOSEN_ROW = 10; }
    harness::time_function("./example.txt", &part1);
    unsafe { CHOSEN_ROW = 2000000; }
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    unsafe { MAX_DIM = 4000000; }
    harness::time_function("./data.txt", &part2);
}
