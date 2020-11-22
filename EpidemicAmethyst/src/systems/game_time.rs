use amethyst::{
    core::{
        SystemDesc,
        timing::Time
    },
    ecs::{System, World, WriteExpect, SystemData, Read},
};
use time;

pub enum PlayStateEnum {
    Paused,
    InGame
}

#[derive(Default)]
pub struct GameTimeSystem {}

impl<'s> System<'s> for GameTimeSystem {
    type SystemData = (
        WriteExpect<'s, time::PrimitiveDateTime>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut game_datetime, timing): Self::SystemData) {
        println!(
            "Game time is: {}, {}",
            game_datetime.date(),
            game_datetime.time()
        );

        *game_datetime += timing.absolute_time();
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
