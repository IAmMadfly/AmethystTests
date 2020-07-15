use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::Entity,
    prelude::*,
    ui::UiCreator,
    renderer::{
        sprite::{SpriteRender, Sprite, SpriteSheet},
        Camera, ImageFormat, Texture},
    window::{ScreenDimensions}
};

#[derive(Default, Debug)]
pub struct WelcomeState {
    ui_handle:  Option<Entity>
}

impl SimpleState for WelcomeState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        println!("Loading world!");
        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/welcome.ron", ())));
        println!("Finished loading world! Is ui loaded? {}", self.ui_handle.is_some());
    }

    fn handle_event(
        &mut self, 
        _data: StateData<'_, GameData<'_, '_>>, 
        _event: StateEvent
    ) -> SimpleTrans {
        let world = _data.world;

        Trans::None
    }
}