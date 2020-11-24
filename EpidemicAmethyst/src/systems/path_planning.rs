use amethyst::{
    core::{Transform, SystemDesc},
    ecs::{
        Join, ReadExpect,
        System, WriteStorage,
        ReadStorage, World, SystemData
    },
};

use time::{PrimitiveDateTime, Weekday};

use crate::tools::path_planner;
use crate::infection::population;
use crate::systems::game_time::PlayStateEnum;

fn get_weekday_index(weekday: Weekday) -> u8 {
    match weekday {
        Weekday::Monday => 0,
        Weekday::Tuesday => 1,
        Weekday::Wednesday => 2,
        Weekday::Thursday => 3,
        Weekday::Friday => 4,
        Weekday::Saturday => 5,
        Weekday::Sunday => 6,
    }
}

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
        ReadStorage<'s, population::Job>,
        WriteStorage<'s, population::InBuilding>,
        WriteStorage<'s, population::Traveling>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, PlayStateEnum>,
        ReadExpect<'s, path_planner::PathPlanner>,
        ReadExpect<'s, PrimitiveDateTime>
    );

    fn run(
        &mut self,
        (
            people,
            jobs,
            mut inbuildings,
            mut travels,
            mut transforms,
            game_play_state,
            path_planner,
            datetime
        ): Self::SystemData
    ) {
        for (person, job, inbuilding) in (&people, &jobs, &mut inbuildings).join() {
            let weekday_index = get_weekday_index(datetime.weekday());

            
            
        }
    }
}

#[derive(Default)]
pub struct PathPlanningSystemDesc {}

impl<'a, 'b> SystemDesc<'a, 'b, PathPlanningSystem> for PathPlanningSystemDesc {
    fn build(self, world: &mut World) -> PathPlanningSystem {
        <PathPlanningSystem as System<'_>>::SystemData::setup(world);

        world.insert(path_planner::PathPlanner::new((0,0)));

        PathPlanningSystem::default()
    }
}