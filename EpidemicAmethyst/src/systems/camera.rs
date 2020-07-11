use amethyst::{
    winit,
    core::{Transform},
    ecs::{Component, Join, Read, ReadExpect, ReadStorage, System, VecStorage, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
    window::ScreenDimensions
};

pub struct CameraMovementSystem {
    prev_mouse_pos:         Option<(f32, f32)>,
    camera_center:          [f32; 2],
    camera_width:           f32,                 // Only half width to reduce future calcs
    camera_width_min:       f32
}

impl Default for CameraMovementSystem {
    fn default() -> Self {
        CameraMovementSystem {
            prev_mouse_pos:     None,
            camera_center:      [0.0,0.0],
            camera_width:       300.0,
            camera_width_min:   120.0
        }
    }
}

impl Component for CameraMovementSystem {
    type Storage = VecStorage<Self>;
}

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>
    );

    fn run(&mut self, (mut camera, mut transforms, input_handler, screen_dim): Self::SystemData) {
        // Change width value
        self.camera_width = self.camera_width + input_handler.mouse_wheel_value(false) * 5.0;
            
        // Change camera position
        if input_handler.mouse_button_is_down(winit::MouseButton::Left) {
            if let Some(prev_mouse_pos) = self.prev_mouse_pos {
                if let Some(curr_mouse_pos) = input_handler.mouse_position() {
                    let (pre_x, pre_y) = prev_mouse_pos;
                    let (cur_x, cur_y) = curr_mouse_pos;

                    self.camera_center[0] = self.camera_center[0] - (cur_x - pre_x);
                    self.camera_center[1] = self.camera_center[1] - (cur_y - pre_y);
                }
            }
            self.prev_mouse_pos = input_handler.mouse_position();
        } else {
            self.prev_mouse_pos = None;
        }

        if self.camera_width < self.camera_width_min {
            self.camera_width = self.camera_width_min;
        }

        let top =       self.camera_center[1] + (self.camera_width);
        let bottom =    self.camera_center[1] - (self.camera_width);
        let left =      self.camera_center[0] - (self.camera_width * screen_dim.aspect_ratio());
        let right =     self.camera_center[0] + (self.camera_width * screen_dim.aspect_ratio());

        for (camera, _transform) in (&mut camera, &mut transforms).join() {

            if let Some(ortho_view) = camera.projection_mut().as_orthographic_mut() {
                ortho_view.set_bottom_and_top(
                    bottom,
                    top
                );
                ortho_view.set_left_and_right(
                    left,
                    right
                );

                //println!(
                //    "top: {}, Right: {}, Bottom: {}, Left: {}", 
                //    ortho_view.top(),
                //    ortho_view.right(),
                //    ortho_view.bottom(),
                //    ortho_view.left()
                //);
            }
        }
    }
}