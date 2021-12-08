pub fn day08(input: String) -> String {
    let encodes:Vec<(&str, &str)> = input.lines()
        .map(|l| l.split(" | ").collect() )
        .map(|i: Vec<&str> | (i[0], i[1])).collect();
    
    let mut appear_1478 = 0;
    let mut all_sum = 0;
    for e in encodes {
        let (encode, digits) = e;
        
        let digits = digits.split(' ')
            .map(|d| get_digit_for_str(encode, d));
        
        appear_1478 += digits.clone().filter(|i| *i == 1 || *i == 4 || *i == 7 || *i == 8).count();
        
        assert_eq!(digits.clone().count(), 4, "Should have exactly 4 digits???");

        let dv:Vec<i32> = digits.collect();
        let (d1, d2, d3, d4) = (dv[0], dv[1], dv[2], dv[3]);
        all_sum += d1 * 1000 + d2 * 100 + d3 * 10 + d4;
    }
    
    return format!("1,4,7,8 appears {} times, total of all values {}", appear_1478, all_sum);
}

fn get_sum_for_str(encodes: &str, in_str: &str) -> i32 {
    let mut total: i32 = 0;
    for ch in in_str.chars() {
        total += encodes.chars().filter(|c| *c == ch).count() as i32;
    }

    return total;
}

fn get_digit_for_str(encodes: &str, digit_str: &str) -> i32 {
    let digit_sums = vec![42, 17, 34, 39, 30, 37, 41, 25, 49, 45];

    let total = get_sum_for_str(encodes, digit_str);
    let digit = digit_sums.iter().position(|i| *i == total).unwrap();
    
    return digit as i32;
}