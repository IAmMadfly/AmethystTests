use amethyst::{
    core::{Transform, SystemDesc},
    ecs::{
        Join, Read, ReadExpect,
        System, WriteStorage,
        ReadStorage, World, SystemData
    },
};

use time::{PrimitiveDateTime, Weekday};

use crate::tools::path_planner;
use crate::infection::population::{self, BuildingContainerComponent};
use crate::infection::buildings;
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
        ReadStorage<'s, population::Job>,
        ReadStorage<'s, buildings::Building>,
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
            buildings,
            mut inbuildings,
            mut travels,
            mut transforms,
            game_play_state,
            path_planner,
            datetime
        ): Self::SystemData
    ) {
        
        for (person, job, inbuilding) in (&people, &jobs, &mut inbuildings).join() {
            if job.work_active(*datetime) {
                let start_building = buildings
                    .get(inbuilding.get_building())
                    .expect("Failed to get Building for InBuilding");
                
                let end_building = buildings
                    .get(job.get_building())
                    .expect("Failed to get Building for Job");
                
                let start_location_format = (
                    start_building.location().block_x() as usize,
                    start_building.location().block_y() as usize
                );
                let end_location_format = (
                    end_building.location().block_x() as usize,
                    end_building.location().block_y() as usize
                );

                println!(
                    "Start location: {},\t End Location: {}",
                    start_building.location(),
                    end_building.location()
                );
                
                let path = path_planner
                    .plan_path(
                        start_location_format,
                        end_location_format
                    );
                
                if let Some(valid_path) = path {
                    println!("Found a valid path to work!");
                } else {
                    println!("Failed to get path to work!");
                }
            }
            
        }
    }
}

#[derive(Default)]
pub struct PathPlanningSystemDesc {}

impl<'a, 'b> SystemDesc<'a, 'b, PathPlanningSystem> for PathPlanningSystemDesc {
    fn build(self, world: &mut World) -> PathPlanningSystem {
        <PathPlanningSystem as System<'_>>::SystemData::setup(world);

        world.insert(path_planner::PathPlanner::new((0,0)));
        println!("Returning PathPlanningSystem!");
        PathPlanningSystem::default()
    }
}