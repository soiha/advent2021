use std::i32::MIN;

pub fn day01(input: String) -> String {
    let mut last: i32 = MIN;
    let mut incs: i32 = 0;
    
    for s in input.lines() {
        let cur:i32 = s.trim().parse().expect(format!("Could not parse {} as an integer", s).as_str());
        
        if last > MIN && cur > last {
            incs += 1;
        }
        
        last = cur;
    }
    
    let mut window_incs: i32 = 0;
    let v: Vec<&str> = input.lines().collect();
    let ints: Vec<i32> = v.iter().map(|s| s.trim()).map(|s| s.parse().unwrap()).collect();
    for i in 0..v.len() {
        if i + 3 < v.len() {
            let acc_a: i32 = ints[i..i+3].iter().sum();
            let acc_b: i32 = ints[i+1..i+4].iter().sum();

            if acc_b > acc_a {
                window_incs += 1;
            }
        }
    }
    
    return format!("Increases: {} , {} with sliding window 3.", incs, window_incs);
}