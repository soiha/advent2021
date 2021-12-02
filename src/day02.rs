pub fn day02(input: String) -> String {
    let v: Vec<&str> = input.lines().collect();
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    let mut pos_aimed: i32 = 0;
    let mut depth_aimed: i32 = 0;
    
    for s in v {
        let mut pair = s.split_ascii_whitespace();
        let command = pair.next().unwrap();
        let amount: i32 = pair.next().unwrap().parse().unwrap();
        
        match command {
            "forward" => {
                pos += amount;
                
                pos_aimed += amount;
                depth_aimed += aim * amount;
            },

            "up" => {
                depth -= amount;
                aim -= amount;
            },
            
            "down" => {
                depth += amount;
                aim += amount;
            },
            
            _ => println!("Unknown command {}", command),
        }
    }
    
    return format!("\nInitial: Pos {} Depth {} Multiplied {}\nAimed:   Pos {} Depth {} Multiplied {}\n", 
                   pos, depth, pos * depth, pos_aimed, depth_aimed, pos_aimed * depth_aimed);
}