use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::Lines;
use harness;

#[derive(Clone, Debug, Eq)]
enum Data {
    Raw(usize),
    List(Vec<Data>),
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Raw(l0), Self::Raw(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Raw(l0), Self::List(r0)) => &vec![Data::Raw(*l0)] == r0,
            (Self::List(l0), Self::Raw(r0)) => &vec![Data::Raw(*r0)] == l0,
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Data::List(list) => {
                match other {
                    Data::List(other_list) => {
                        if list.len() == 0 {
                            if other_list.len() == 0 {
                                Some(Ordering::Equal)
                            } else {
                                Some(Ordering::Less)
                            }
                        } else {
                            let mut li = list.iter();
                            let mut oli = other_list.iter();
                            
                            let mut ret_val: Option<Ordering>;

                            loop {
                                let left_item = li.next();
                                let right_item = oli.next();

                                if left_item.is_none() && right_item.is_none() {
                                    ret_val = Some(Ordering::Equal);
                                    break;
                                }

                                if left_item.is_none() && right_item.is_some() {
                                    ret_val = Some(Ordering::Less);
                                    break;
                                }

                                if left_item.is_some() && right_item.is_none() {
                                    ret_val = Some(Ordering::Greater);
                                    break;
                                }

                                let left_item = left_item.unwrap();
                                let right_item = right_item.unwrap();

                                ret_val = left_item.partial_cmp(right_item);

                                if ret_val != Some(Ordering::Equal) {
                                    break;
                                }
                            }

                            ret_val
                        }
                    },
                    Data::Raw(other_raw) => {
                        self.partial_cmp(&Data::List(vec![Data::Raw(*other_raw)]))
                    }
                }
            },
            Data::Raw(raw) => {
                match other {
                    Data::List(_) => {
                        Data::List(vec![Data::Raw(*raw)]).partial_cmp(other)
                    },
                    Data::Raw(other_raw) => {
                        raw.partial_cmp(other_raw)
                    },
                }
            },
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(x) => x,
            None => panic!("partial cmp returned none"),
        }
    }
}

fn parse_packet(s: &str) -> Data {
    let s = &s[1..s.len()-1];
    let mut current_list: Vec<Data> = Vec::new();
    let mut working_packet: String = String::new();
    let mut working_number: String = String::new();
    let mut recording_packet: usize = 0;
    for c in s.chars() {
        match c {
            '[' => {
                recording_packet += 1;
                if recording_packet > 0 {
                    working_packet.push(c);
                    continue;
                }
            }
            ']' => {
                if recording_packet > 0 {
                    recording_packet -= 1;
                    working_packet.push(c);
                    
                    if recording_packet == 0 {
                        // finish packet
                        current_list.push(
                            parse_packet(&working_packet)
                        );
                        working_packet = String::new();
                    }
                }

                // finish number if is one
                if !working_number.is_empty() {
                    let num = usize::from_str_radix(working_number.as_str(), 10).unwrap();
                    current_list.push(Data::Raw(num));
                    working_number = String::new();
                }
            },
            ',' => {
                if recording_packet > 0 {
                    working_packet.push(c);
                    continue;
                }

                // finish number if is one
                if !working_number.is_empty() {
                    let num = usize::from_str_radix(working_number.as_str(), 10).unwrap();
                    current_list.push(Data::Raw(num));
                    working_number = String::new();
                }
            },
            x => {
                if recording_packet > 0 {
                    working_packet.push(c);
                    continue;
                }

                // start/continue number
                working_number.push(x);
            }
        };
    }

    if !working_number.is_empty() {
        let num = usize::from_str_radix(working_number.as_str(), 10).unwrap();
        current_list.push(Data::Raw(num));
    }
    
    let ret_val = Data::List(current_list);
    ret_val
}

fn compare_data(left: &Data, right: &Data) -> bool {
    let in_right_order = match left.cmp(&right) {
        Ordering::Greater => false,
        _ => true,
    };

    in_right_order
}

fn part1(lines: &mut Lines) -> usize {
    let mut index = 1;
    let mut sum = 0;
    loop {
        let mut it = lines.take(3);
        let left = it.next();
        let right = it.next();
        it.next();

        if left.is_none() || right.is_none() {
            break;
        }

        let left = left.unwrap();
        let right = right.unwrap();

        let left_packet = parse_packet(left);
        let right_packet = parse_packet(right);
        
        if compare_data(&left_packet, &right_packet) {
            sum += index;
        }
        index += 1;
    }
    sum
}

fn part2(lines: &mut Lines) -> usize {
    let mut packets: Vec<Data> = Vec::new();
    let divider_packets = vec![
        parse_packet("[[2]]"),
        parse_packet("[[6]]"),
    ];
    packets.push(divider_packets[0].clone());
    packets.push(divider_packets[1].clone());

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let packet = parse_packet(line);
        packets.push(packet);
    }
    packets.sort();
    let mut result: usize = 1;
    for (index, packet) in packets.iter().enumerate() {
        if divider_packets.contains(packet) {
            result *= index + 1;
        }
    }
    result
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}
