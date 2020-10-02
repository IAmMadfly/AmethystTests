use amethyst::{
    ecs::prelude::Entity,
    input,
    prelude::*,
    ui::{UiCreator, UiFinder, UiEvent, UiEventType, UiButtonBuilder, UiButton},
    winit::VirtualKeyCode
};

use crate::states::game::PlayStateEnum;

pub struct PauseState {
    ui_handle:      Option<Entity>
}

impl Default for PauseState {
    fn default() -> Self {
        PauseState{
            ui_handle:      None
        }
    }
}

impl SimpleState for PauseState {
    fn on_start(
        &mut self, 
        _data: StateData<'_, GameData<'_, '_>>
    ) {
        *_data.world.write_resource::<PlayStateEnum>() = PlayStateEnum::Paused;
        println!("Entered PauseState");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        *_data.world.write_resource::<PlayStateEnum>() = PlayStateEnum::Paused;
        println!("Exit PauseState");
    }

    fn update(
        &mut self, 
        _data: &mut StateData<'_, GameData<'_, '_>>
    ) -> SimpleTrans {
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        _event: StateEvent
    ) -> SimpleTrans {
        let _world = _data.world;

        match &_event {
            StateEvent::Window(window_event) => {
                if input::is_key_down(&window_event, VirtualKeyCode::Escape) {
                    return Trans::Pop
                }
                Trans::None
            },
            _ => Trans::None
        }
    }

}