use amethyst::{
    assets::{Asset, AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, VecStorage},
    prelude::*,
    renderer::{
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        Camera, ImageFormat, Texture},
};

pub struct GameState;

struct House {
    people:         Vec<People>,
    location:       Point
}

struct Point {
    x:              i32,
    y:              i32
}

struct People {
    home:           House
}

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Game starting!");
        println!("Reading map file");
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        //println!("Handling event!");
        Trans::None
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        //println!("Updating GameState!");
        Trans::None
    }
}

mod map {
    use amethyst::{
        assets::{Asset, AssetStorage, Handle, Loader},
        core::transform::Transform,
        prelude::*,
        ecs::{VecStorage, prelude::World},
        renderer::{
            sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
            Camera, ImageFormat, Texture},
    };
    use tiled;
    
}