//noinspection ALL

pub fn day03(input: String) -> String { 
    let v: Vec<&str> = input.lines().collect();
    
    /*
    let v = vec![
        "00100","11110","10110","10111","10101","01111","00111","11100","10000","11001","00010","01010",
    ];
     */
    
    // only up to 32 bits here but welp
    let mut zerocounts: Vec<i32> = vec![0; 32];
    let mut onecounts: Vec<i32> = vec![0; 32];
    
    for s in &v {
        let bits = s.len();
        for i in (0..bits).rev() {
            let ch = s.as_bytes()[i] as char;
            match ch {
                '1' => { onecounts[bits-i-1] += 1; }
                '0' => { zerocounts[bits-i-1] += 1; }
                _ => { println!("Illegal character: {}", ch)}
            }
        }
    }
    
    let mut gamma = 0;
    let mut epsilon = 0;
    
    for i in 0..32 {
        let c0 = zerocounts[i];
        let c1 = onecounts[i];
        
        if c0 > 0 || c1 > 0 {
            if c0 == c1 {
                println!("Equal amounts for bit {} ???", i);
            }
            
            if c1 > c0 {
                gamma |= 1 << i;
            } else {
                epsilon |= 1 << i;
            }
            
        }
    }

    let mut ox_values = v.clone();
    let mut co_values = v.clone();
    
    for i in (0..ox_values[0].len()).rev() {
        let c0_ox = ox_values.iter().filter(|x| x.as_bytes()[x.len()-1-i] as char == '0').count() as i32;
        let c1_ox = ox_values.iter().filter(|x| x.as_bytes()[x.len()-1-i] as char == '1').count() as i32;
        let c0_co = co_values.iter().filter(|x| x.as_bytes()[x.len()-1-i] as char == '0').count() as i32;
        let c1_co = co_values.iter().filter(|x| x.as_bytes()[x.len()-1-i] as char == '1').count() as i32;

        if c0_ox > 0 || c1_ox > 0 {
            let ox_keep = if c1_ox >= c0_ox { '1' } else { '0' };
            
            if ox_values.len() > 1 {
                ox_values.retain(|x| x.as_bytes()[x.len()-1-i] as char == ox_keep);
            }
        }
        
        if c0_co > 0 || c1_co > 0 {
            let co_keep = if c0_co <= c1_co { '0' } else { '1' };

            if co_values.len() > 1 {
                co_values.retain(|x| x.as_bytes()[x.len()-1-i] as char == co_keep);
            }
        }
    }
    
    let final_ox;
    let final_co;
    if ox_values.len() == 1 && co_values.len() == 1 {
        final_ox = ox_values[0];
        final_co = co_values[0];
    } else {
        panic!("Remaining {} ox values and {} co values???", ox_values.len(), co_values.len());
    }
    
    let ox_result: i32 = bin_str_to_int(final_ox); 
    let co_result: i32 = bin_str_to_int(final_co);
    
    return format!("Gamma {} Epsilon {} Multiplied {}\nOxygen Generator {} CO2 Scrub {} Multiplied {}", 
                   gamma, epsilon, gamma * epsilon, ox_result, co_result, ox_result * co_result);
}

fn bin_str_to_int(s: &str) -> i32 {
    return (0..s.as_bytes().len()).map(|i| if s.as_bytes()[s.len()-1-i] as char == '1' { 1 << i as i32 } else {0}).sum();
}