use amethyst::{
    ecs::prelude::Entity,
    assets::Loader,
    input,
    prelude::*,
    ui::{UiCreator, UiFinder, UiEvent, UiText, UiEventType, UiButtonBuilder, UiButton, LineMode, Anchor, TtfFormat},
    winit::VirtualKeyCode
};

use crate::infection::population;

pub struct ViewHomeState {
    people_text:    UiText
}

impl ViewHomeState {
    fn new(home: population::Home, world: &World) -> Self {
        let font_handle = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource()
        );
        let ui_text = UiText::new(
            font_handle, 
            home.families.len().to_string(), 
            [1.0, 1.0, 1.0, 0.7], 
            25.0, 
            LineMode::Single, 
            Anchor::Middle);

        ViewHomeState {
            people_text:        ui_text
        }
    }
}

impl SimpleState for ViewHomeState {
    fn on_start(
        &mut self, 
        _data: StateData<'_, GameData<'_, '_>>
    ) {

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