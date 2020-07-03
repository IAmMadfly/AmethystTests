use amethyst::{
    winit,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Component, Join, Read, ReadStorage, System, VecStorage, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::camera::Camera
};

use crate::game::{};

#[derive(Default)]
pub struct CameraMovementSystem {
    prev_mouse_pos:     Option<(f32, f32)>,
    zoom_level:         f32
}

impl Component for CameraMovementSystem {
    type Storage = VecStorage<Self>;
}

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut camera, mut transforms, input_handler): Self::SystemData) {

        for (camera, transform) in (&mut camera, &mut transforms).join() {

            if let Some(ortho_view) = camera.projection_mut().as_orthographic_mut() {
                let mouse_scroll_val = input_handler.mouse_wheel_value(false);
                if (self.zoom_level > 0.0 || mouse_scroll_val > 0.0) {
                    let scroll_multiplier = 5.0;

                    ortho_view.set_bottom_and_top(
                        ortho_view.bottom() - scroll_multiplier*mouse_scroll_val,
                        ortho_view.top() + scroll_multiplier*mouse_scroll_val
                    );
                    ortho_view.set_left_and_right(
                        ortho_view.left() - scroll_multiplier*mouse_scroll_val,
                        ortho_view.right() + scroll_multiplier*mouse_scroll_val
                    );
                    self.zoom_level = self.zoom_level + 0.05*mouse_scroll_val;
                }
            }

            if input_handler.mouse_button_is_down(winit::MouseButton::Left) {
                if let Some(prev_mouse_pos) = self.prev_mouse_pos {
                    if let Some(curr_mouse_pos) = input_handler.mouse_position() {
                        let (pre_x, pre_y) = prev_mouse_pos;
                        let (cur_x, cur_y) = curr_mouse_pos;
                        
                        transform.move_left((cur_x - pre_x) * self.zoom_level);
                        transform.move_up((cur_y - pre_y) * self.zoom_level);
                    }
                }
                self.prev_mouse_pos = input_handler.mouse_position();
            } else {
                self.prev_mouse_pos = None;
            }

        }
    }
}