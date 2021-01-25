use std::hash::{Hash, Hasher};
use pathfinding::prelude::{astar, absdiff};

fn get_new_map(size: (usize, usize)) -> Vec<Vec<PathPoint>> {
    let mut vec = Vec::new();
    for y_index in 0..size.1 {
        vec.insert(
            y_index, 
            Vec::new()
        );
        for x_index in 0..size.0 {
            vec[y_index].insert(
                x_index,
                PathPoint::new(x_index, y_index)
            );
        }
    }
    vec
}

pub struct PathPoint {
    x:      u32,
    y:      u32,
    valid:  bool
}

impl PathPoint {
    fn new(x: usize, y: usize) -> PathPoint {
        PathPoint {
            x:      x as u32,
            y:      y as u32,
            valid:  false
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    fn get_i64_location(&self) -> (f32, f32) {
        (
            self.x() as f32,
            self.y() as f32
        )
    }

    fn distance_to(&self, other: &PathPoint) -> f32 {
        let first_location = self.get_i64_location();
        let second_location = other.get_i64_location();

        (
            (first_location.0 - second_location.0).powi(2) +
            (first_location.1 - second_location.1).powi(2)
        ).sqrt()
    }

    fn x_y_distance_to(&self, other: &PathPoint) -> u32 {
        (absdiff(self.x(), other.x()) + absdiff(self.y(), other.y())) as u32
    }
}

impl Clone for PathPoint {
    fn clone(&self) -> Self {
        PathPoint {
            x:      self.x,
            y:      self.y,
            valid:  self.valid
        }
    }
}

impl Hash for PathPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for PathPoint {
    fn eq(&self, other: &PathPoint) -> bool {
        if self.x == other.x && self.y == other.y {
            return true
        }
        false
    }
}
impl Eq for PathPoint {}

pub struct PathPlanner {
    map:    Vec<Vec<PathPoint>>
    // map[y][x], This makes it easier for debugging purposes, 
    // as printing in x direction i s natural
}

impl PathPlanner {
    pub fn new(size: (usize, usize)) -> Self {
        println!("New map size: {}, {}", size.0, size.1);

        let planner = 
            PathPlanner {
                map:    get_new_map(size)
            };

        planner
    }

    pub fn new_map(&mut self, size: (usize, usize)) {
        self.map = get_new_map(size);
    }

    pub fn add_path_blocks(&mut self, location: (u32, u32), size: (u32, u32)) {
        let start_x     = location.0 as usize;
        let start_y     = location.1 as usize;
        let end_x       = (location.0 + size.0) as usize;
        let end_y       = (location.1 + size.1) as usize;

        println!("Adding allowable path to map. Location: {:?}, Size: {:?}", location, size);

        for x  in start_x..end_x {
            for y in start_y..end_y {
                self.map[y][x]
                    .valid = true;
            }
        }
    }

    fn distance(&self, start: (usize, usize), end: (usize, usize)) -> f32 {
        let x_diff = (start.0 as f32 - end.0 as f32).abs();
        let y_diff = (start.1 as f32 - end.1 as f32).abs();
        
        ((x_diff.powi(2) + y_diff.powi(2))).sqrt()
    }

    fn get_point(&self, point: &PathPoint, x: i64, y: i64) -> Option<PathPoint> {
        self.map
            .get((point.y as i64 + y) as usize)
            .and_then(
                |y_vec| y_vec.get((point.x as i64 + x) as usize).and_then(
                    |pp| if pp.valid {
                        Some(pp.clone())
                    } else {
                        None
                    })
                )
    }

    fn find_succesors(&self, point: &PathPoint) -> Vec<(PathPoint, u32)> {
        // let mut succcess_vec = Vec::new();
        
        let possible_success = {
            let mut vec = Vec::new();
            if let Some(point) = self.get_point(point, -1, 0) {
                vec.push((point, 1));
            }
            if let Some(point) = self.get_point(point, 0, -1) {
                vec.push((point, 1));
            }
            if let Some(point) = self.get_point(point, 0, 1) {
                vec.push((point, 1));
            }
            if let Some(point) = self.get_point(point, 1, 0) {
                vec.push((point, 1));
            }
            
            vec
        };

        possible_success
    }

    pub fn plan_path(&self, start: (usize, usize), end: (usize, usize)) -> Option<Vec<PathPoint>> {
        let start = PathPoint{
            valid:  true, 
            x:      start.0 as u32, 
            y:      start.1 as u32
        };

        let end = PathPoint {
            valid:  true,
            x:      end.0 as u32,
            y:      end.1 as u32
        };

        let result = astar(
            &start, 
            |p| self.find_succesors(p), 
            |p| p.x_y_distance_to(&end), 
            |p| end.eq(p)
        );
        
        if let Some(path) = result {
            return Some(path.0)
        }
        None
    }

    pub fn _debug_map(&self) {
        for y in &self.map {
            for x in y {
                if x.valid {
                    print!("x");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    pub fn _debug_map_with_start_end(&self, start: (u32, u32), end: (u32, u32)) {
        for y in &self.map {
            for x in y {
                if x.valid && ((x.x() == start.0 && x.y() == start.1) || (x.x() == end.0 && x.y() == end.1)) {
                    print!("#");
                } else {
                    if ((x.x() == start.0 && x.y() == start.1) || (x.x() == end.0 && x.y() == end.1)) {
                        print!("o");
                    } else {
                        if x.valid {
                            print!("x");
                        } else {
                            print!(".")
                        }
                    }
                }
            }
            println!();
        }
    }
}