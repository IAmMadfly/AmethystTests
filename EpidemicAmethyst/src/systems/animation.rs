use amethyst::{
    ecs::{Component, Join, Read, ReadExpect, System, DenseVecStorage, WriteStorage},
    core::timing::Time,
    renderer::SpriteRender
};

use crate::systems::game_time::PlayStateEnum;

#[derive(Clone)]
pub struct AnimatedSprite {
    pub animation_data:     Vec<tiled::Frame>,
    pub curr_index:         usize,
    pub curr_duration:      std::time::Duration
}

impl AnimatedSprite {
    pub fn new(data: Vec<tiled::Frame>) -> Self {
        if data.is_empty() {
            panic!("Cannot construct AnimatedSprite with no animation data");
        }
        AnimatedSprite {
            animation_data:     data,
            curr_index:         0,
            curr_duration:      std::time::Duration::new(0, 0)
        }
    }
}

impl Component for AnimatedSprite {
    type Storage = DenseVecStorage<Self>;
}

pub struct SpriteAnimationSystem {
    prev_time:      std::time::SystemTime
}

impl Default for SpriteAnimationSystem {
    fn default() -> Self {
        SpriteAnimationSystem{
            prev_time:      std::time::SystemTime::now()
        }
    }
}

impl<'s> System<'s> for SpriteAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, AnimatedSprite>,
        Read<'s, Time>,
        ReadExpect<'s, PlayStateEnum>
    );

    fn run(&mut self, (mut sprite_renders, mut sprite_animation, time, play_state): Self::SystemData) {
        if let PlayStateEnum::InGame = *play_state {
            for (sprite_render, animation) in (&mut sprite_renders, &mut sprite_animation).join() {
                animation.curr_duration += time.delta_time();
    
                if let Some(frame) = animation.animation_data.get(animation.curr_index) {
                    if animation.curr_duration.as_millis() >= (frame.duration as u128) {
                        let mut new_index: usize = animation.curr_index + 1;
                        if new_index == animation.animation_data.len() {
                            new_index =     0;
                        }
                        
                        sprite_render.sprite_number =   animation.animation_data[new_index].tile_id as usize;
                        animation.curr_duration =       std::time::Duration::new(0, 0);
                        animation.curr_index =          new_index;
                    }
                } else {
                    println!("Error getting Vector value!!");
                }
            }
        }
    }
}
