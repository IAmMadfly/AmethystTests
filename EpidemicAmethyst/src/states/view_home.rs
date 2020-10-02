use amethyst::{
    assets::Loader,
    ecs::Entity,
    input,
    prelude::*,
    ui::{UiTransform, UiText, LineMode, Anchor, TtfFormat},
    winit::VirtualKeyCode
};
use crate::states::game::PlayStateEnum;
use crate::infection::population;

pub struct ViewHomeState {
    people_count_ui:        Option<Entity>
}

impl ViewHomeState {
    pub fn new(home: &population::Home) -> Self {
        ViewHomeState {
            people_count_ui:    None
        }
    }
}

impl SimpleState for ViewHomeState {
    fn on_start(
        &mut self, 
        _data: StateData<'_, GameData<'_, '_>>
    ) {

        let font_handle = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource()
        );

        let ui_text_tranform = UiTransform::new(
            String::from("people_text_transform"), 
            Anchor::Middle, 
            Anchor::Middle,
            0.0,
            0.0, 
            0.0, 
            100.0, 
            30.0
        );

        let ui_text = UiText::new(
            font_handle, 
            home.families.len().to_string(), 
            [1.0, 1.0, 1.0, 0.7], 
            25.0, 
            LineMode::Single, 
            Anchor::Middle);
        
        let people_count_ui = world.create_entity()
            .with(ui_text_tranform)
            .with(ui_text)
            .build();


        *_data.world.write_resource::<PlayStateEnum>() = PlayStateEnum::Paused;
        println!("Entered ViewHomeState");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        *_data.world.write_resource::<PlayStateEnum>() = PlayStateEnum::InGame;
        println!("Exiting ViewHomeState");
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