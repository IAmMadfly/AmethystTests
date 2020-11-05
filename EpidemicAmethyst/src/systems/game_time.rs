use amethyst::{
    core::Transform,
    ecs::{Component, Join, Read, ReadExpect, System, VecStorage, WriteStorage, ReadStorage},
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
    window::ScreenDimensions
};

pub struct GameTimeSystem {

}

impl<'s> System<'s> for GameTimeSystem {
    type SystemData = (

    );

    fn run(&mut self, _data: Self::SystemData) {
        
    }
}
