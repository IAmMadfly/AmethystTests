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
    prev_mouse_pos:     Option<(f32, f32)>
}

impl Component for CameraMovementSystem {
    type Storage = VecStorage<Self>;
}

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (camera, mut transforms, input_handler): Self::SystemData) {

        for (camera, transform) in (&camera, &mut transforms).join() {
            //transform.move_up(5.0);

            if input_handler.mouse_button_is_down(winit::MouseButton::Left) {
                if let Some(prev_mouse_pos) = self.prev_mouse_pos {
                    if let Some(curr_mouse_pos) = input_handler.mouse_position() {
                        let (pre_x, pre_y) = prev_mouse_pos;
                        let (cur_x, cur_y) = curr_mouse_pos;
                        
                        transform.move_left(cur_x - pre_x);
                        transform.move_up(cur_y - pre_y);
                    }
                }
                self.prev_mouse_pos = input_handler.mouse_position();
            } else {
                self.prev_mouse_pos = None;
            }

        }
    }
}