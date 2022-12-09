use std::{path::Path, fs::File, io, io::{BufReader, Read}, time::{Instant}, str::Lines};

pub fn get_input(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buff = String::new();
    let mut buff_reader = BufReader::new(file);
    buff_reader.read_to_string(&mut buff)?;
    Ok(buff)
}

pub fn time_function<T: std::fmt::Display>(path: &str, f: &dyn Fn(&mut Lines) -> T) {
    let str = match get_input(Path::new(path)) {
        Ok(s) => s,
        Err(_) => panic!("Error getting input"),
    };
    let mut lines = str.lines();
    let start_time = Instant::now();
    let result = f(&mut lines);
    let elapsed_time = start_time.elapsed();
    println!("\nResult:\t\t{}\nDuration:\t{:?}", result, elapsed_time);
}