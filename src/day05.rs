const GRID_MAX_EXTENT: usize = 1000;

struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn from_input(input: &str) -> Point {
        let elems: Vec<&str> = input.split(',').collect();
        let xs = elems[0].parse().unwrap();
        let ys = elems[1].parse().unwrap();
     
        return Point {
            x: xs,
            y: ys
        };
    }
}

struct LineSeg {
    start: Point,
    end: Point,
}

impl LineSeg {
    pub fn from_input(input: &str) -> LineSeg {
        let elems: Vec<&str> = input.split_ascii_whitespace().collect();
        let p1 = elems[0];
        let p2 = elems[2];
        
        return LineSeg {
            start: Point::from_input(p1),
            end: Point::from_input(p2)
        };
    }
    
    pub fn draw_to(&self, grid: &mut Vec<Vec<i32>>) {
        // hello Bresenham you old fart

        let mut x0 = self.start.x;
        let mut y0 = self.start.y;
        let x1 = self.end.x;
        let y1 = self.end.y;
        
        let dx = i32::abs(x1 - x0);
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -i32::abs(y1 - y0);
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            grid[x0 as usize][y0 as usize] += 1;
            if x0 == x1 && y0 == y1 {
                break;
            }
            
            let e2 = 2*err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

pub fn day05(input: String) -> String {
    let mut grid = vec![vec![0; GRID_MAX_EXTENT]; GRID_MAX_EXTENT];
    let mut grid2 = vec![vec![0; GRID_MAX_EXTENT]; GRID_MAX_EXTENT];
    let lines: Vec<LineSeg> = input.lines().map(|line| LineSeg::from_input(line)).collect();
    
    lines.iter()
        .filter(|seg| seg.start.x == seg.end.x || seg.start.y == seg.end.y )
        .for_each(|seg| seg.draw_to(&mut grid));

    lines.iter().for_each(|seg| seg.draw_to(&mut grid2));
    
    let straight_intersects = grid.iter().flatten().filter(|elem| **elem > 1).count();
    let all_intersects = grid2.iter().flatten().filter(|elem| **elem > 1).count();
    
    return format!("Only straight lines: {} intersections, all lines {} intersections", 
                   straight_intersects, all_intersects);
}