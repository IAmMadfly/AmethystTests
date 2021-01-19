use amethyst::{
    core::{
        SystemDesc,
        timing::Time
    },
    ecs::{System, World, WriteExpect, SystemData, Read, ReadExpect},
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
        ReadExpect<'s, PlayStateEnum>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut game_datetime, playstate, timing): Self::SystemData) {
        println!(
            "Game time is: {}, {}",
            game_datetime.date(),
            game_datetime.time()
        );

        if let PlayStateEnum::InGame = *playstate {
            *game_datetime += timing.absolute_time();
        }
    }
}

#[derive(Default)]
pub struct GameTimeSystemDesc {}

impl<'a, 'b> SystemDesc<'a, 'b, GameTimeSystem> for GameTimeSystemDesc {
    fn build(self, world: &mut World) -> GameTimeSystem {
        <GameTimeSystem as System<'_>>::SystemData::setup(world);

        world.insert(time::PrimitiveDateTime::new(
            time::Date::try_from_ymd(2020, 1, 1).expect("Filed to get date!"),
            time::Time::try_from_hms(8, 30, 0).expect("Failed to get time!'")
        ));
        world.insert(PlayStateEnum::Paused);

        GameTimeSystem::default()
    }
}
