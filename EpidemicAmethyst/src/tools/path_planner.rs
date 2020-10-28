
use pathfinding::prelude::{astar};


struct PathPoint {
    x:      usize,
    y:      usize
}

impl PathPoint {
    fn new(x: usize, y: usize) -> Self {
        PathPoint {
            x,
            y
        }
    }

    fn distance(&self, other: &PathPoint) -> f32 {
        let x_diff = (self.x as f32 - other.x as f32).abs();
        let y_diff = (self.y as f32 - other.y as f32).abs();
        
        ((x_diff.powi(2) + y_diff.powi(2))).sqrt()
    }
}
pub struct PathPlanner {
    map:    Vec<PathPoint> 
    // map[y][x], This makes it easier for debugging purposes, 
    // as printing in x direction i s natural
}

impl PathPlanner {
    pub fn default() -> Self {
        PathPlanner {
            map:    Vec::new()
        }
    }

    pub fn add_path_blocks(&mut self, location: (u32, u32), size: (u32, u32)) {
        let start_x     = location.0 as usize;
        let start_y     = location.1 as usize;
        let end_x       = (location.0 + size.0) as usize;
        let end_y       = (location.1 + size.1) as usize;

        println!("Adding allowable path to map. Location: {:?}, Size: {:?}", location, size);

        for x  in start_x..end_x {
            for y in start_y..end_y {
                self.map.push(PathPoint::new(x,y));
            }
        }
    }

    pub fn plan_path(&self, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        


        Some(Vec::<(usize, usize)>::new())
    }

    pub fn _debug_map(&self) {
        
    }
}