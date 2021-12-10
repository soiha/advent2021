pub fn day10(input: String) -> String {
    let lines:Vec<&str> = input.lines().collect();
    let mut complete:Vec<String> = vec![];
    
    let scores = vec![3, 57, 1197, 25137];
    let opens = "([{<";
    let closes = ")]}>";
    let mut error_score = 0;
    
    for chunk in lines {
        let mut queue: Vec<char> = vec![];
        let mut did_error = false;

        for ch in chunk.chars() {
            let opener = opens.chars().position(|c| c == ch);
            let closer = closes.chars().position(|c| c == ch);

            if opener.is_some() {
                queue.push(ch);
            }

            if closer.is_some() {
                let last = queue.last().unwrap();
                if closer.unwrap() != opens.chars().position(|c| c == *last).unwrap() {
                    error_score += scores[closer.unwrap()];
                    did_error = true;
                    break;
                } else {
                    queue.pop();
                }
            }
        }
        
        if !did_error {
            let mut compl = String::new();
            
            while queue.len() > 0 {
                let cc = queue.pop().unwrap();
                let opener_pos = opens.chars().position(|c| c == cc).unwrap();
                compl.push(closes.chars().nth(opener_pos).unwrap());
            }
            complete.push(compl.to_owned());
        }
    }
    
    let mut scores:Vec<i64> = vec![];
    for c in complete {
        let mut score = 0;
        c.chars().for_each(|ch| {
            score *= 5;
            let ch_score:i64 = closes.chars().position(|pc| ch == pc ).unwrap() as i64 + 1;
            score += ch_score;
        });
        
        scores.push(score);
    }
    
    scores.sort();
    let winner = scores[scores.len() / 2];
        
    return format!("Total error score {}, completion score {}", error_score, winner);
}