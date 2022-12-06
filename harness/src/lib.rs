use std::{path::Path, fs::File, io, io::{BufReader, Read}, time::{Instant}};

pub fn get_input(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buff = String::new();
    let mut buff_reader = BufReader::new(file);
    buff_reader.read_to_string(&mut buff)?;
    Ok(buff)
}

pub fn time_function(path: &Path, f: &dyn Fn(&Path) -> i32) {
    let start_time = Instant::now();
    let result = f(path);
    let elapsed_time = start_time.elapsed();
    println!("\nResult:\t\t{}\nDuration:\t{:?}", result, elapsed_time);
}
