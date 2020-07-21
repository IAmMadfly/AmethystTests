use amethyst::{
    ecs::prelude::Entity,
    prelude::*,
    ui::{UiCreator, UiButtonBuilder, UiButton}
};


#[derive(Default, Debug)]
pub struct WelcomeState {
    ui_handle:  Option<Entity>
}

impl SimpleState for WelcomeState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        
        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/welcome.ron", ())));
        println!("Finished loading world! Is ui loaded? {}", self.ui_handle.is_some());
    }

    fn handle_event(
        &mut self, 
        _data: StateData<'_, GameData<'_, '_>>, 
        _event: StateEvent
    ) -> SimpleTrans {
        let _world = _data.world;

        Trans::None
    }
}