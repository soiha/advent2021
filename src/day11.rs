const GRID_MAX_EXTENT: usize = 10;

pub fn day11(input: String) -> String {
    let mut grid = vec![vec![0; GRID_MAX_EXTENT]; GRID_MAX_EXTENT];
    let mut flashes_at:Vec<(usize, usize)> = vec![];
    let mut did_flash:Vec<(usize, usize)> = vec![];
    
    input.lines().enumerate().for_each(|(ln, l)| {
        l.chars().enumerate().for_each(|(pos, ch)| {
            grid[pos][ln] = ch.to_digit(10).unwrap();
        });
    });

    let mut steps = 0;
    let mut flash_count = 0;
    
    loop {
        steps += 1;
        assert!( steps < 10000, "No sync after 10000 steps???");
        
        for y in 0..GRID_MAX_EXTENT {
            for x in 0..GRID_MAX_EXTENT {
                grid[x][y] += 1;
                if grid[x][y] > 9 {
                    flashes_at.push((x,y));
                }
            }
        }

        did_flash.clear();
        
        while flashes_at.len() > 0 {
            let flash = flashes_at.pop().unwrap();
            if !did_flash.contains(&flash) {
                if steps <= 100 { flash_count += 1; }  // don't care after 100 steps
                grid[flash.0][flash.1] = 0;
                did_flash.push(flash);
                
                neighbors(flash).iter().for_each( |(x,y)| {
                    if grid[*x][*y] > 0 { grid[*x][*y] += 1; }
                    if grid[*x][*y] > 9 { flashes_at.push((*x,*y)); }
                });
            }
        }
        
        if grid.iter().flatten().filter( |o| **o == 0 ).count() == GRID_MAX_EXTENT * GRID_MAX_EXTENT { 
            break;
        }
    }
    
    return format!("Total {} flashes at step 100; first step for sync is {}", flash_count, steps);
}

fn neighbors(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let (x,y) = pos;
    let mut ret:Vec<(usize, usize)> = vec![];
    
    if x > 0 && y > 0                 { ret.push((x-1, y-1)); }
    if x > 0                          { ret.push((x-1, y));   }
    if x > 0 && y < GRID_MAX_EXTENT-1 { ret.push((x-1, y+1)); }
    if y > 0                          { ret.push((x,   y-1)); }
    if y < GRID_MAX_EXTENT-1          { ret.push((x,   y+1)); }
    if x < GRID_MAX_EXTENT-1 && y > 0 { ret.push((x+1, y-1)); }
    if x < GRID_MAX_EXTENT-1          { ret.push((x+1, y));   }
    if x < GRID_MAX_EXTENT-1 && y < GRID_MAX_EXTENT-1 { ret.push((x+1, y+1)); }
        
    ret
}
