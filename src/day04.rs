// hardcoded, but who cares
const CARD_ROWS: usize = 5;
const CARD_COLS: usize = 5;

pub fn day04(input: String) -> String {
    let v: Vec<&str> = input.lines().collect();
    let rng: Vec<i32> = v[0].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let mut cards: Vec<Vec<i32>> = vec![];
    let mut marks: Vec<Vec<bool>> = vec![];
    let mut winners: Vec<i32> = vec![];
    
    let mut card_count = 0;
    let mut i = 2;
    
    while i < v.len() {
        cards.push(vec![]);
        marks.push(vec![]);
        
        for s in v[i..i+CARD_ROWS].iter() {
            for s2 in s.split_ascii_whitespace() {
                cards[card_count].push(s2.parse::<i32>().unwrap());
                marks[card_count].push(false);
            }
        }

        card_count += 1;
        i += CARD_ROWS + 1;
    }
    
    let mut win_col;
    let mut win_row;
    let mut first_win_card_index: i32 = -1;
    let mut first_win_called_number = -1;
    let mut first_win_unmarked_sum = 0;
    let mut last_win_called_number = -1;
    
    for n in rng {
        for ci in 0..cards.len() {
            if winners.contains(&(ci as i32)) {
                continue;
            }
            
            cards[ci].iter().position(|x| x == &n).map(|pos| {
                marks[ci][pos] = true;
            } );
            
            win_col = check_win_cols(&marks[ci]);
            win_row = check_win_rows(&marks[ci]);
            
            if win_col != -1 || win_row != -1 {
                if winners.contains(&(ci as i32)) == false {
                    winners.push(ci as i32);
                    if winners.len() == cards.len() {
                        last_win_called_number = n;
                        break;
                    }
                }
                
                if first_win_card_index == -1 {
                    first_win_card_index = ci as i32;
                    first_win_called_number = n;
                    first_win_unmarked_sum = sum_unmarked(&cards[first_win_card_index as usize], &marks[first_win_card_index as usize]);
                }
            }
        }
    }
    
    let last_win_card_index = winners.last().unwrap();
    let last_board_score = sum_unmarked(&cards[*last_win_card_index as usize], &marks[*last_win_card_index as usize]) * last_win_called_number;
    return format!("First winner card {} sum is {} last call {} score {}\nLast winner card {} last call {} score {}", 
                   first_win_card_index +1, first_win_unmarked_sum, first_win_called_number, first_win_unmarked_sum * first_win_called_number, 
                   last_win_card_index + 1, last_win_called_number, last_board_score);
}

fn get_card_element_at<T: Copy>(card: &Vec<T>, row: usize, col: usize) -> T {
    return card[row * CARD_ROWS + col];
}

fn check_win_rows(marks: &Vec<bool>) -> i32 {  // returns the index of the winning row
    let mut i = 0;
    while i < marks.len() {
        if marks[i..i+CARD_COLS].iter().all( |x| x == &true ) {
            return (i % CARD_ROWS) as i32;
        }
        i += CARD_COLS;
    }
    
    return -1;
}

fn check_win_cols(marks: &Vec<bool>) -> i32 {
    for i in 0..CARD_COLS {
        if (0..CARD_ROWS).all(|j| get_card_element_at(&marks, j, i) == true ) {
            return i as i32;
        }
    }
    
    return -1;
}

fn sum_unmarked(card: &Vec<i32>, marks: &Vec<bool>) -> i32 {
    let mut sum = 0;
    
    for i in 0..card.len() {
        if marks[i] == false { sum += card[i]; }        
    }
    
    return sum;
}