pub fn day06(input: String) -> String {
    let mut fish: Vec<i64> = vec![0; 9];
    
    input.split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .for_each(|n| fish[n as usize] += 1 );
    
    let sum_80:i64 = do_count(80, &fish);
    let sum_256 = do_count(256, &fish);
    
    return format!("Totals: 80 days: {}  256 days: {}", sum_80, sum_256);
}

fn do_count(days: i32, in_fish: &Vec<i64>) -> i64 {
    let mut fish = in_fish.clone();
    
    for _ in 0..days {
        let d0 = fish[0];
        fish.remove(0);
        fish[6] += d0;
        fish.push(d0);
    }

    return fish.iter().sum();
}
