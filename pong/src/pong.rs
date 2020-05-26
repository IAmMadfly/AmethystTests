
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

use amethyst::{
    //assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Pong;

pub enum Side {
    Left,
    Right
}

pub struct Paddle {
    pub side:   Side,
    pub width:  f32,
    pub height: f32
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width:  PADDLE_WIDTH,
            height: PADDLE_HEIGHT
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_camera(world: &mut World) {

    let mut transform = Transform::default();

    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialise_paddles(world: &mut World) {
    let mut left_transform =    Transform::default();
    let mut right_transform =   Transform::default();

    let y = ARENA_HEIGHT / 2.0;

    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Create left plank entity
    world.create_entity()
         .with(Paddle::new(Side::Left))
         .with(left_transform)
         .build();
    
    // Create left plank entity
    world.create_entity()
         .with(Paddle::new(Side::Right))
         .with(right_transform)
         .build();
}

//fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {

//}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        world.register::<Paddle>();
        initialise_paddles(world);
    }
}