use std::env;
use std::fs;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let input = fs::read_to_string(file)
        .expect(format!("Could not read input file {}", file).as_str());
    let result = crate::day01::day01(input);
    
    print!("Result is: {}", result);
}
