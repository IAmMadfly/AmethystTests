use amethyst::{
    winit,
    prelude::World,
    core::{Transform, math},
    ecs::{Component, Join, Read, ReadExpect, System, VecStorage, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
    window::ScreenDimensions
};

use crate::states::game;

use crate::systems::game_time::PlayStateEnum;

pub struct CameraMovementSystem {
    prev_mouse_pos:         Option<(f32, f32)>,
    camera_scale:           f32
}

impl Default for CameraMovementSystem {
    fn default() -> Self {
        CameraMovementSystem {
            prev_mouse_pos:     None,
            camera_scale:       1.0
        }
    }
}

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Option<Read<'s, PlayStateEnum>>
    );

    fn run(&mut self, (mut camera, mut transforms, input_handler, game_play_state): Self::SystemData) {
        let mut run_system = false;
        if let Some(play_state) = game_play_state {
            match *play_state {
                PlayStateEnum::Paused => run_system = false,
                PlayStateEnum::InGame => run_system = true
            }
        }

        if !run_system {
            return
        }

        let mouse_pos = input_handler.mouse_position();
        
        // Change width value
        self.camera_scale = self.camera_scale + input_handler.mouse_wheel_value(false) * -0.05;
        
        let mut x_change = 0.0;
        let mut y_change = 0.0;

        // Change camera position
        if input_handler.mouse_button_is_down(winit::MouseButton::Left) {
            if let Some(prev_mouse_pos) = self.prev_mouse_pos {
                if let Some(curr_mouse_pos) = mouse_pos {
                    let (pre_x, pre_y) = prev_mouse_pos;
                    let (cur_x, cur_y) = curr_mouse_pos;

                    x_change = pre_x - cur_x;
                    y_change = cur_y - pre_y;
                    
                }
            }
            self.prev_mouse_pos = mouse_pos;
        } else {
            self.prev_mouse_pos = None;
        }

        for (_camera, transform) in (&mut camera, &mut transforms).join() {
            transform.append_translation_xyz(
                x_change, 
                y_change,
                0.0
            );

            transform.set_scale(
                math::Vector3::new(
                        self.camera_scale,
                        self.camera_scale,
                        1.0
                    )
            );
        }
    }
}