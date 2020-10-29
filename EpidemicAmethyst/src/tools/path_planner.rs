use std::rc;
use pathfinding::prelude::{astar};


struct PathPoint {
    valid:  bool
}

impl Default for PathPoint {
    fn default() -> Self {
        PathPoint {
            valid:  false
        }
    }
}

pub struct PathPlanner {
    map:    Vec<Vec<PathPoint>>
    // map[y][x], This makes it easier for debugging purposes, 
    // as printing in x direction i s natural
}

impl PathPlanner {
    pub fn new(size: (usize, usize)) -> Self {
        let mut y_vec = Vec::with_capacity(size.1);

        let mut planner = 
            PathPlanner {
                map:    y_vec
            };

        for y_index in 0..(size.1 - 1) {
            planner.map[y_index] = Vec::with_capacity(size.0);

            for x_index in 0..(size.0 - 1) {
                planner.map[y_index][x_index] = PathPoint::default();
            }
        }

        planner
    }

    pub fn add_path_blocks(&mut self, location: (u32, u32), size: (u32, u32)) {
        let start_x     = location.0 as usize;
        let start_y     = location.1 as usize;
        let end_x       = (location.0 + size.0) as usize;
        let end_y       = (location.1 + size.1) as usize;

        println!("Adding allowable path to map. Location: {:?}, Size: {:?}", location, size);

        for x  in start_x..end_x {
            for y in start_y..end_y {
                self.map[x][y]
                    .valid = true;
            }
        }
    }

    fn distance(&self, start: (usize, usize), end: (usize, usize)) -> f32 {
        let x_diff = (start.0 as f32 - end.0 as f32).abs();
        let y_diff = (start.1 as f32 - end.1 as f32).abs();
        
        ((x_diff.powi(2) + y_diff.powi(2))).sqrt()
    }

    pub fn plan_path(&self, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        


        Some(Vec::<(usize, usize)>::new())
    }

    pub fn _debug_map(&self) {
        
    }
}