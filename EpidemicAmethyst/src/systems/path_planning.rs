use amethyst::{
    core::Transform,
    ecs::{Component, Join, Read, ReadExpect, System, VecStorage, WriteStorage, ReadStorage},
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
    window::ScreenDimensions
};

use crate::states::game;
use crate::infection::population;
use crate::systems::game_time::PlayStateEnum;

pub struct PathPlanningSystem {

}

impl Default for PathPlanningSystem {
    fn default() -> Self {
        PathPlanningSystem {
            
        }
    }
}

impl<'s> System<'s> for PathPlanningSystem {
    type SystemData = (
        ReadStorage<'s, population::Person>,
        WriteStorage<'s, population::InBuilding>,
        WriteStorage<'s, population::Traveling>,
        Option<Read<'s, PlayStateEnum>>
    );

    fn run(&mut self, (people, mut inbuildings, mut travels, game_play_state): Self::SystemData) {
        for (person, inbuilding) in (&people, &mut inbuildings).join() {

        }
    }
}