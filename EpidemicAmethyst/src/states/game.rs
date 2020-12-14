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
        rendy::wsi::winit::MouseButton,
        Camera
    },
    window::{ScreenDimensions},
    winit::VirtualKeyCode
};

use crate::states;
use crate::infection;

pub struct GameState {
    map:            tiled::Map,
    houses:         Vec<Entity>,
    workplaces:     Vec<Entity>,
    people:         Vec<Entity>,
    camera:         Entity
}

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        
        println!("-- Game is starting!! --");
        let world = _data.world;

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
                    let world_location = (
                        (world_point.x/32.0).floor() as u32,
                        (world_point.y/32.0).floor() as u32
                    );

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
        Trans::None
    }
}

impl GameState {
    pub fn new(
        map:            tiled::Map, 
        houses:         Vec<Entity>,
        workplaces:     Vec<Entity>,
        people:         Vec<Entity>,
        camera:         Entity
    ) -> GameState {
            GameState {
                map,
                houses,
                workplaces,
                people,
                camera
            }

    }

    fn check_home_location(&self, location: (u32, u32), world: &World) -> Option<Entity> {
        for home_ent in self.houses.clone() {
            let home_comp = world.read_component::<infection::buildings::Building>();
            
            let home = home_comp
                .get(home_ent)
                .expect("Failed to get Size for home");

            if (home.location().block_x() <= location.0) && (home.location().block_y() <= location.1) {
                if ((home.location().block_x() + home.size().0) > location.0) & ((home.location().block_y() + home.size().1) > location.1) {
                    return Some(home_ent)
                }
            }
        }
        None
    }
}


