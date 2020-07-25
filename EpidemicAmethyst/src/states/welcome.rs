use amethyst::{
    ecs::prelude::Entity,
    input,
    prelude::*,
    ui::{UiCreator, UiFinder, UiEvent, UiEventType, UiButtonBuilder, UiButton},
    winit::VirtualKeyCode
};

use crate::states::game;

#[derive(Default, Debug)]
pub struct WelcomeState {
    ui_handle:      Option<Entity>,
    start_butt:     Option<Entity>
}

impl SimpleState for WelcomeState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        
        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/welcome.ron", ())));
        
        game::load_game_map(world);
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.start_butt.is_none() {
            state_data.world.exec(|ui_finder: UiFinder<'_>| {
                self.start_butt = ui_finder.find("start")
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self, 
        _data: StateData<'_, GameData<'_, '_>>, 
        event: StateEvent
    ) -> SimpleTrans {
        let _world = _data.world;

        match event {
            StateEvent::Window(window_event) => {
                if input::is_close_requested(&window_event) {
                    Trans::Quit
                } else if input::is_key_down(&window_event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target
            }) => {
                if Some(target) == self.start_butt {
                    println!("Starting game!!");
                    return Trans::Switch(Box::new(game::GameState::default()))
                }
                Trans::None
            }
            _ => Trans::None
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {

        if let Some(root_entity) = self.ui_handle {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to delete Start screen");
        }

        self.ui_handle =    None;
        self.start_butt =   None;
    }
}