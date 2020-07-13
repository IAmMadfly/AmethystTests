use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::Entity,
    prelude::*,
    renderer::{
        sprite::{SpriteRender, Sprite, SpriteSheet},
        Camera, ImageFormat, Texture},
    window::{ScreenDimensions}
};

pub struct WelcomeState {
    ui_handle:  Option<Entity>
};

impl SimpleState for WelcomeState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;

        self.ui_handle = 
            Some(world.exec())
    }
}