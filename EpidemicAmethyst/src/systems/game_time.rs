use amethyst::{
    core::SystemDesc,
    ecs::{System, SystemData, World},
};
use time;

pub enum PlayStateEnum {
    Paused,
    InGame
}

#[derive(Default)]
pub struct GameTimeSystem {}

impl<'s> System<'s> for GameTimeSystem {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        
    }
}

#[derive(Default)]
pub struct GameTimeSystemDesc {}

impl<'a, 'b> SystemDesc<'a, 'b, GameTimeSystem> for GameTimeSystemDesc {
    fn build(self, world: &mut World) -> GameTimeSystem {
        <GameTimeSystem as System<'_>>::SystemData::setup(world);

        world.insert(time::PrimitiveDateTime::new(
            time::date!(2020-01-01),
            time::Time::midnight()
        ));
        world.insert(PlayStateEnum::Paused);

        GameTimeSystem::default()
    }
}
