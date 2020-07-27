use amethyst::{
    ecs::prelude::Entity,
    input,
    prelude::*,
    ui::{UiCreator, UiFinder, UiEvent, UiEventType, UiButtonBuilder, UiButton},
    winit::VirtualKeyCode
};

use crate::states::game;
use crate::states::passer;
use std::thread;

#[derive(Debug)]
pub struct WelcomeState {
    game_loader:    passer::Passer<game::GameState>,
    ui_handle:      Option<Entity>,
    start_butt:     Option<Entity>
}

impl Default for WelcomeState {
    fn default() -> Self {
        WelcomeState {
            game_loader:    passer::Passer::new(game::GameState::default()),
            ui_handle:      None,
            start_butt:     None
        }
    }
}

impl SimpleState for WelcomeState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        
        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/welcome.ron", ())));
        
        //game::load_game_map(world);
        if let Some(loader) = &self.game_loader.item {
            loader.borrow_mut().load_map(world);
        } else {
            let game_state = game::GameState::default();
            game_state.load_map(world);
            self.game_loader = passer::Passer::new(game_state);
        }
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
                    return Trans::Switch(
                        Box::new(
                            self.game_loader
                                .return_val("Error getting game from RefCell")
                                .expect("Failed to get preloaded game from option")
                        )
                    )
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