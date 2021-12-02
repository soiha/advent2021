use std::env;
use std::fs;
use std::fmt;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let input = fs::read_to_string(file)
        .expect(format!("Could not read input file {}", file).as_str());
    crate::day01::day01(input);
}
