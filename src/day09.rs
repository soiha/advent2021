const GRID_MAX_EXTENT: usize = 100;

pub fn day09(input: String) -> String {
    let mut grid:Vec<Vec<i32>> = vec![vec![-1; GRID_MAX_EXTENT]; GRID_MAX_EXTENT];
    let mut lowpoints:Vec<(usize, usize)> = vec![];
    
    input.lines().enumerate().for_each( |l| {
        let (y, s) = l;
        s.chars().enumerate().for_each(|c| grid[c.0][y] = c.1.to_digit(10).unwrap() as i32 );
    });
    
    for y in 0..GRID_MAX_EXTENT {
        for x in 0..GRID_MAX_EXTENT {
            let n = neighbors(&grid, (x,y));
            let p = grid[x][y];
            
            if p == -1 { continue; }
            
            if n.iter().all(|e| { *e > p || *e == -1 }) {
                lowpoints.push((x,y))
            }
        }
    }
    
    let risk_total:i32 = lowpoints.iter()
        .map(|n| grid[n.0][n.1]+1 ).sum();
    
    let mut basins:Vec<Vec<(usize,usize)>> = 
        lowpoints.into_iter()
            .map(|lp| map_basin(&grid, lp, &vec![]))
            .collect();
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    
    assert!(basins.len() >= 3, "Need three or more basins!");
    
    let b3_total = basins[0].len() * basins[1].len() * basins[2].len();
    return format!("Low point total risk value {}, total of largest 3 basin sizes multiplied {}", 
                   risk_total, b3_total);
}

fn neighbors( grid: &Vec<Vec<i32>>, pos: (usize, usize) ) -> Vec<i32> {
    let (x,y) = pos;
    
    let u:i32 = if y <= 0 { -1 } else { grid[x][y-1] };
    let d:i32 = if y >= GRID_MAX_EXTENT-1 { -1 } else { grid[x][y+1] };
    let l:i32 = if x <= 0 { -1 } else { grid[x-1][y] };
    let r:i32 = if x >= GRID_MAX_EXTENT-1 { -1 } else { grid[x+1][y] };
    
    vec![u, d, l, r]
}

fn map_basin( grid: &Vec<Vec<i32>>, pos: (usize, usize), visited: &Vec<(usize, usize)> ) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let p = grid[x][y];
    
    if p >= 9 {
        return vec![];
    }
    
    let mut ret = vec![pos];
    let mut visited_new = visited.clone();
    visited_new.push(pos);
    
    let u = if y <= 0 || visited_new.contains( &(x, y-1) ) { vec![] } else { 
        let m = map_basin(grid, (x, y-1), &visited_new); 
        visited_new.extend(m.iter()); 
        m 
    };
    
    let d = if y >= GRID_MAX_EXTENT-1 || visited_new.contains( &(x, y+1) ) { vec![] } else { 
        let m = map_basin(grid, (x, y+1), &visited_new); 
        visited_new.extend(m.iter()); 
        m
    };
    
    let l = if x <= 0 || visited_new.contains( &(x-1, y) ) { vec![] } else { 
        let m = map_basin(grid, (x-1, y), &visited_new); 
        visited_new.extend(m.iter()); 
        m 
    };
    
    let r = if x >= GRID_MAX_EXTENT-1 || visited_new.contains( &(x+1, y) ) { vec![] } else { 
        let m = map_basin(grid, (x+1, y), &visited_new); 
        visited_new.extend(m.iter()); 
        m 
    };
    
    ret.extend(u.iter());
    ret.extend(d.iter());
    ret.extend(l.iter());
    ret.extend(r.iter());
    
    ret
}