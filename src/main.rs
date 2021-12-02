use std::env;
use std::fs;

#[allow(dead_code)]

mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let input = fs::read_to_string(file)
        .expect(format!("Could not read input file {}", file).as_str());
    let result = crate::day02::day02(input);
    
    print!("Result is: {}", result);
}
