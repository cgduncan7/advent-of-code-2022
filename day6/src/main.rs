use std::str::Lines;
use harness;

struct DataStreamBuffer {
    data: String
}

impl DataStreamBuffer {
    fn find_start_of_packet_marker_index(&self, distinct_chars: usize) -> usize {
        let mut prev_chars: Vec<char> = vec![];
        let mut index = 0;
        for ch in self.data.chars() {
            index += 1;
            if prev_chars.contains(&ch) {
                // remove until ch
                let mut new_chars: Vec<char> = vec![];
                for nch in prev_chars.iter().skip_while(|x| *x != &ch).skip(1) {
                    new_chars.push(*nch);
                }
                prev_chars = new_chars;
            }
            prev_chars.push(ch);

            if prev_chars.len() == distinct_chars {
                break;
            }
        }

        index
    }
}

fn part1(lines: &mut Lines) -> String {
    let mut results: Vec<String> = Vec::new();
    for line in lines {
        results.push(DataStreamBuffer { data: String::from(line) }.find_start_of_packet_marker_index(4).to_string());
    }
    
    results.join("; ")
}

fn part2(lines: &mut Lines) -> String {
    let mut results: Vec<String> = Vec::new();
    for line in lines {
        results.push(DataStreamBuffer { data: String::from(line) }.find_start_of_packet_marker_index(14).to_string());
    }
    
    results.join("; ")
}

fn main() {
    harness::time_function("./example.txt", &part1);
    harness::time_function("./data.txt", &part1);
    harness::time_function("./example.txt", &part2);
    harness::time_function("./data.txt", &part2);
}