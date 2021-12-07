type CostFunc = fn(i64, i64) -> i64;

pub fn day07(input: String) -> String {
    let crabs: Vec<i64> = input.split(',')
        .map(|n| n.parse::<i64>().unwrap()).collect();
    
    let d1 = do_calc(&crabs, day1_cost);
    let d2 = do_calc(&crabs, day2_cost);
    
    let (_, sm_cost_01, sm_pos_01) = d1;
    let (_, sm_cost_02, sm_pos_02) = d2;
    
    return format!("Smallest cost {} to pos {} with simple cost. Cost {} to pos {} with complex cost.", 
                   sm_cost_01, sm_pos_01, sm_cost_02, sm_pos_02);
}

fn do_calc(crabs: &Vec<i64>, cost_func: CostFunc) -> (usize, i64, i64) {
    let mut smallest_res: (usize, i64, i64) = (0, i64::MAX, -1);  // (index, cost, pos)

    for crab in crabs.iter().enumerate() {

        let (index, pos) = crab;
        let sum = crabs.iter().map( |c| cost_func(*pos+1, *c) ).sum();
        
        if sum < smallest_res.1 {
            smallest_res = (index, sum, *pos);
        }
    }
    
    smallest_res
}

fn day1_cost(from: i64, to: i64) -> i64 {
    i64::abs(from - to)
}

fn day2_cost(from: i64, to: i64) -> i64 {
    let dist = day1_cost(from, to);
    (dist*(dist+1)) / 2
}