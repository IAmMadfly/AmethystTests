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
    home:                   Entity,
    people_count_ui:        Option<Entity>
}

impl ViewHomeState {
    pub fn new(home: Entity) -> Self {
        ViewHomeState {
            home,
            people_count_ui:    None
        }
    }
}

impl SimpleState for ViewHomeState {
    fn on_start(
        &mut self, 
        data: StateData<'_, GameData<'_, '_>>
    ) {
        let mut world = data.world;

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
            "Working on it".to_owned(),  //  world.read_component::<population::Building>()
                                        //    .get(self.home)
                                        //    .expect("Failed to get Home component for Home").len().to_string(), 
            [1.0, 1.0, 1.0, 0.7], 
            25.0, 
            LineMode::Single, 
            Anchor::Middle);
        
        let people_count_ui = world.create_entity()
            .with(ui_text_tranform)
            .with(ui_text)
            .build();
        
        self.people_count_ui = Some(people_count_ui);

        *world.write_resource::<PlayStateEnum>() = PlayStateEnum::Paused;
        println!("Entered ViewHomeState");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Exiting ViewHomeState");

        if let Some(people_count_entity) = self.people_count_ui {
            _data.world.delete_entity(people_count_entity).expect("Failed to delete UI Entity");
            self.people_count_ui = None;
        }
        *_data.world.write_resource::<PlayStateEnum>() = PlayStateEnum::InGame;
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