use amethyst::{
    core::{
        transform::Transform,
        math::{Vector2, Point3},
        timing
    },
    prelude::*,
    ecs::prelude::Entity,
    input::{self, InputEvent},
    renderer::{
        rendy::{wsi::winit::MouseButton},
        Camera
    },
    window::{ScreenDimensions},
    winit::VirtualKeyCode
};

// use std::{
//     collections::HashMap,
//     io::BufReader,
//     path::Path,
//     fs::File
// };

use crate::states;
use crate::infection;
use crate::tools;

pub enum PlayStateEnum {
    Paused,
    InGame
}

pub struct GameState {
    map:            tiled::Map,
    houses:         Vec<Entity>,
    workplaces:     Vec<Entity>,
    people:         Vec<Entity>,
    camera:         Entity,
    play_state:     PlayStateEnum,
    path_planner:   tools::path_planner::PathPlanner
}

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        
        println!("-- Game is starting!! --");
        let world = _data.world;

        world.insert(PlayStateEnum::InGame);
        //world.register::<CameraMovementSystem>();

        world.write_resource::<timing::Time>().set_time_scale(10.0);
    }

    fn handle_event(&mut self, state_data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        //println!("Handling event!");
        match &event {
            StateEvent::Window(window_event) => {
                if input::is_key_down(&window_event, VirtualKeyCode::Escape) {
                    Trans::Push(Box::new(states::pause::PauseState::default()))
                    //Trans::Quit
                } else {
                    Trans::None
                }
            },
            StateEvent::Input(InputEvent::MouseButtonReleased(MouseButton::Left)) => {

                let input_handler = 
                    state_data
                    .world
                    .read_resource::<input::InputHandler<input::StringBindings>>();
                
                let screen_dimentions =
                    state_data
                    .world
                    .read_resource::<ScreenDimensions>();

                if let Some(position) = input_handler.mouse_position() {
                    let screen_point = Point3::new(position.0, position.1, 0.0);
                    let screen_size = Vector2::new(
                        screen_dimentions.width(), 
                        screen_dimentions.height()
                    );

                    let camera = state_data.world.read_component::<Camera>();
                    let transform = state_data.world.read_component::<Transform>();

                    let transform_comp = 
                        transform.get(self.camera).expect("Failed to get Transform Component for Camera");
                    let camera_comp = 
                        camera.get(self.camera).expect("Failed to get Camera Component for Camera");
                    
                    let world_point = camera_comp.screen_to_world_point(
                        screen_point, 
                        screen_size, 
                        transform_comp
                    );
                    let world_location = ((world_point.x/32.0).floor(), (world_point.y/32.0).floor());

                    if let Some(home) = self.check_home_location(world_location, state_data.world) {
                        println!("Is a home location!!");
                        println!("Home has {} maximum occupants", 
                            state_data.world.read_component::<infection::buildings::MaxOccupants>()
                                .get(home).expect("Failed to get building!").get_max_occupants()
                        );

                        return Trans::None;
                        //return Trans::Push(Box::new(states::view_home::ViewHomeState::new(home)))
                    }
                }
                
                Trans::None
            },
            _ => Trans::None
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        //println!("Updating GameState!");
        let time = state_data.world.read_resource::<timing::Time>();

        println!("Time is: {}, vs {}", 
            time.absolute_time().as_secs_f32(), 
            time.absolute_real_time().as_secs_f32()
        );

        Trans::None
    }
}

impl GameState {
    pub fn new(
        map:            tiled::Map, 
        houses:         Vec<Entity>,
        workplaces:     Vec<Entity>,
        people:         Vec<Entity>,
        camera:         Entity,
        play_state:     PlayStateEnum,
        path_planner:   tools::path_planner::PathPlanner) -> GameState {
            GameState {
                map,
                houses,
                workplaces,
                people,
                camera,
                play_state,
                path_planner
            }

    }

    fn check_home_location(&self, location: (f32, f32), world: &World) -> Option<Entity> {
        for home_ent in self.houses.clone() {
            let home_loc_comp = world.read_component::<infection::buildings::Location>();
            let home_comp = world.read_component::<infection::buildings::Building>();

            let home_location = home_loc_comp
                .get(home_ent)
                .expect("Failed to get location for Home");
            
            let home = home_comp
                .get(home_ent)
                .expect("Failed to get Size for home");

            if (home_location.x() <= location.0) && (home_location.y() <= location.1) {
                if ((home_location.x() + home.size[0]) > location.0) & ((home_location.y() + home.size[1]) > location.1) {
                    return Some(home_ent)
                }
            }
        }
        None
    }
}


