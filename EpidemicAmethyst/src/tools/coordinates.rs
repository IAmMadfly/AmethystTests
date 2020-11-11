
use amethyst::core::Transform;

pub trait GridLocation {
    fn grid_xy(&self) -> (f32, f32);

    fn set_grid_xy(&mut self, x: f32, y: f32);
}

impl GridLocation for Transform {
    fn grid_xy(&self) -> (f32, f32) {
        (self.translation()[0]/32.0, self.translation()[0]/32.0)
    }

    fn set_grid_xy(&mut self, x: f32, y: f32) {
        let trans = self.translation_mut();
        trans[0] = x*32.0;
        trans[1] = y*32.0;
    }
}