
#[path ="tools.rs"] pub mod tools;

pub trait Element {
    fn move_position(&mut self, x: i32, y: i32);

    fn get_position(&self) -> Point;
}

pub struct Point {
    pub x:          i32,
    pub y:          i32
}

pub struct Object {
    pub point:      Point,
    pub z:          u32,
    pub size:       tools::Size
}

impl Copy for Point { }

impl Clone for Point {
    fn clone(&self) -> Point {
        Point {
            x:  self.x,
            y:  self.y
        }
    }
}

impl Element for Object {
    
    fn move_position(&mut self, x: i32, y: i32) {
        self.point.x = self.point.x+x;
        self.point.y = self.point.y+y;
    }

    fn get_position(&self) -> Point {
        self.point.clone()
    }
}