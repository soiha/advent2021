use std::env;
use std::fs;

#[allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let days: Vec<fn(String)->String> = vec![
        crate::day01::day01,
        crate::day02::day02,
        crate::day03::day03,
        crate::day04::day04
    ];
    
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let input = fs::read_to_string(file)
        .expect(format!("Could not read input file {}", file).as_str());
    let result = days[3](input);
    
    print!("{}", result);
}
