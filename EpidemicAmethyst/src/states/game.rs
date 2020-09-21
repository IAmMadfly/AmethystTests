use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{
        transform::Transform,
        math::{Vector3, Point2}
    },
    prelude::*,
    ecs::prelude::{Entity, Component, DenseVecStorage},
    input,
    utils::ortho_camera::{
        CameraOrtho,
        CameraNormalizeMode
    },
    renderer::{
        sprite::{SpriteRender, Sprite, SpriteSheet},
        debug_drawing,
        palette::Srgba,
        Camera, ImageFormat, Texture},
    window::{ScreenDimensions},
    winit::VirtualKeyCode
};

use std::{
    collections::HashMap,
    io::BufReader,
    path::Path,
    fs::File
    };

use tiled::parse;

use crate::states::pause;
use crate::infection;

#[derive(Clone)]
pub struct AnimatedSprite {
    pub animation_data:     Vec<tiled::Frame>,
    pub curr_index:         usize,
    pub curr_duration:      std::time::Duration
}

impl AnimatedSprite {
    fn new(data: Vec<tiled::Frame>) -> Self {
        if data.is_empty() {
            panic!("Cannot construct AnimatedSprite with no animation data");
        }
        AnimatedSprite {
            animation_data:     data,
            curr_index:         0,
            curr_duration:      std::time::Duration::new(0, 0)
        }
    }
}

impl Component for AnimatedSprite {
    type Storage = DenseVecStorage<Self>;
}

pub struct GameState {
    map:            Option<tiled::Map>,
    homes:          Vec<infection::population::Home>,
    linesComponent: debug_drawing::DebugLinesComponent
}

impl Default for GameState {
    fn default() -> Self {
        GameState{
            map:            None,
            homes:          Vec::<infection::population::Home>::new(),
            linesComponent: debug_drawing::DebugLinesComponent::new()
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Game is starting!!");
        let world = _data.world;

        //world.register::<CameraMovementSystem>();

        init_camera(world);
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        //println!("Handling event!");
        match &event {
            StateEvent::Window(window_event) => {
                if input::is_key_down(&window_event, VirtualKeyCode::Escape) {
                    Trans::Push(Box::new(pause::PauseState::default()))
                    //Trans::Quit
                } else {
                    Trans::None
                }
            },
            StateEvent::Input(input::InputEvent(input::InputEvent::MouseButtonReleased)) => {
                println!("Got a thing");
                Trans::None
            },
            _ => Trans::None
        }
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        //println!("Updating GameState!");
        Trans::None
    }
}

impl GameState {
    pub fn load_game_map(&mut self, world: &mut World) {
        self.load_map(
            "../Map/MainTown.tmx",
            "../../Map",
            world
        );

        if let Some(map) = &self.map {
            for object_group in &map.object_groups {
                if object_group.name == "Homes" {
                    for home_object in &object_group.objects {
                        self.homes.push(
                            infection::population::Home::new(home_object)
                        );
                    }
                }
            }
        }

        self.linesComponent.add_rectangle_2d(
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 10.0),
            2.1,
            Srgba::new(255.0,255.0,255.0,255.0)
        );
    }

    fn load_map(
        &mut self,
        map_path: &str, 
        image_rel_path: &str, 
        world: &mut World) {
        // Get texture handle for the tileset image
        
        // Load the tiled map
        let file = File::open(&Path::new(map_path)).unwrap();
        let reader = BufReader::new(file);
        let map = parse(reader).unwrap();
    
        let first_tileset = map.tilesets.get(0).expect("No tilesets found!");
        let tile_width = first_tileset.tile_width as i32;
        let tile_height = first_tileset.tile_height as i32;
        
        let _first_image: &tiled::Image = {
            let image_vec = &first_tileset.images;
            if image_vec.len() > 0 {
                image_vec.get(0).expect("Failed to get first image in tileset")
            } else {
                let tiles = &first_tileset.tiles;
                tiles.get(0).expect("Failed to get first tile").images.get(0).expect("Failed to get first image in tile")
            }
        };

        let mut animated_sprite_data: HashMap<u32, AnimatedSprite> =        HashMap::new();
        let mut sprite_sheet_handles: HashMap<u32, Handle<SpriteSheet>> =   HashMap::new();
        for tileset in map.tilesets.iter() {
    
            // Get first image in tileset
            let first_image = {
                if tileset.images.len() > 0 {
                    tileset.images.get(0).expect("Failed to get first image in tileset")
                } else {
                    tileset.tiles
                        .get(0)
                        .expect("Failed to get first tile in tileset")
                        .images
                        .get(0)
                        .expect("Failed to get first image of first tile in tileset")
                }
            };
    
            let tile_width = tileset.tile_width as i32;
            let tile_height = tileset.tile_height as i32;
            let tileset_width = first_image.width;
            let tileset_height = first_image.height;
            let tileset_sprite_columns = tileset_width / tile_width as i32;
            let tileset_sprite_rows = tileset_height / tile_height as i32;
            
            for image in tileset.images.iter() {
    
                let texture_hand = load_texture(
                    [image_rel_path.to_owned(), image.source.clone()].join("/"), 
                    world
                );
                let mut sprites: Vec<Sprite> = Vec::new();
    
                for x in 0..tileset_sprite_rows {
                    for y in 0..tileset_sprite_columns {
                        let tileset_w = *&tileset.images[0].width as u32;
                        let tileset_h = *&tileset.images[0].height as u32;
                        let sprite_w = tile_width as u32;
                        let sprite_h = tile_height as u32;
                        let offset_x = (y * tile_width) as u32;//(y * tile_width) as u32;
                        let offset_y = (x * tile_height) as u32;//(x * tile_height) as u32;
                        // Create a new `Sprite`
                        let sprite = Sprite::from_pixel_values(
                            tileset_w,
                            tileset_h,
                            sprite_w,
                            sprite_h,
                            offset_x,
                            offset_y,
                            [0.0; 2],
                            false,
                            false
                        );
                        
                        sprites.push(sprite);
                    }
                }
    
                let sprite_sheet = SpriteSheet {
                    texture: texture_hand,
                    sprites: sprites
                };
    
                let sprite_sheet_handle = {
                    let loader =                world.read_resource::<Loader>();
                    let sprite_sheet_storage =  world.read_resource::<AssetStorage<SpriteSheet>>();
                    
                    loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
                };
                sprite_sheet_handles.insert(
                    tileset.first_gid,
                    sprite_sheet_handle
                );
            }
    
            for tile in tileset.tiles.iter() {
                for image in tile.images.iter() {
    
                    let texture_hand = load_texture(
                        [image_rel_path.to_owned(), image.source.clone()].join("/"), 
                        world
                    );
    
                    let mut sprites: Vec<Sprite> = Vec::new();
                    // Create a new `Sprite`
                    let sprite = Sprite::from_pixel_values(
                        image.width as u32,
                        image.height as u32,
                        image.width as u32,
                        image.height as u32,
                        0,
                        0,
                        [0.0; 2],
                        false,
                        false
                    );
                    
                    sprites.push(sprite);
    
                    let sprite_sheet = SpriteSheet {
                        texture: texture_hand,
                        sprites: sprites
                    };
    
                    let sprite_sheet_handle = {
                        let loader = world.read_resource::<Loader>();
                        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
                        
                        loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
                    };
    
                    sprite_sheet_handles.insert(
                        tileset.first_gid + tile.id,
                        sprite_sheet_handle
                    );
                }

                // Find animated tiles and implement an animated tile Component for them
                if let Some(tile_animation_data) = &tile.animation {
                    println!("Found an animated tile!");
                    animated_sprite_data.insert(
                        tileset.first_gid + tile.id,
                        AnimatedSprite::new(tile_animation_data.clone())
                    );
                }
            }
        }

        for key in animated_sprite_data.keys() {
            println!("Animated sprites are: {}", key);
        }
            
        // Loop the row first and then the individual tiles on that row
        // and then switch to the next row
        // y = row number
        // x = column number
        // IMPORTANT: Bottom left is 0,0 so the tiles list needs to be reversed with .rev()
        let sprite_sheet_keys: Vec<u32> = {
            let mut keys: Vec<u32> = Vec::new();
            for sprite_sheet_hash in sprite_sheet_handles.keys() {
                keys.push(*sprite_sheet_hash);
            }
            keys.sort();
            keys
        };
        for (l, layer) in map.layers.iter().rev().enumerate() {
            // If the layer is set to invisible, dont render it
            if !layer.visible {
                continue
            }
            for (y, row) in layer.tiles.iter().rev().enumerate().clone() {
                for (x, &tile) in row.iter().enumerate() {
                    // Do nothing with empty tiles
                    if tile.gid == 0 {
                        continue;
                    }
    
                    // Find the sprite_sheet_handle for the right tile.gid.
                    // Due to the way it is organised, we know that if the tile.gid 
                    // is greater or equal to the sprite_sheet_hash key, than that 
                    // is the particular hash key required
                    let sprite_sheet_hash = {
                        let mut num = sprite_sheet_keys.get(sprite_sheet_keys.len() - 1).expect("Failed to get last element in vec").clone();
                        for sprite_sheet_key in sprite_sheet_keys.iter().rev() {
                            if sprite_sheet_key <= &tile.gid {
                                num = sprite_sheet_key.clone();
                                break;
                            }
                        }
                        num
                    };
                    
                    // Sprite for the tile
                    let tile_sprite = SpriteRender {
                        sprite_sheet: sprite_sheet_handles
                            .get(&sprite_sheet_hash)
                            .expect("Got unexpected hash!").clone(),
                        sprite_number: (tile.gid - sprite_sheet_hash) as usize,
                    };
                    
                    // Where we should draw the tile?
                    let mut tile_transform = Transform::default();
                    let x_coord = x * tile_width as usize;
                    let y_coord = (y as f32 * tile_height as f32) + tile_height as f32;
                    // Offset the positions by half the tile size so they're nice and snuggly on the screen
                    // Alternatively could use the Sprite offsets instead: [-32.0, 32.0]. Depends on the use case I guess.
                    let offset_x = tile_width as f32/2.0;
                    let offset_y = -tile_height as f32/2.0;
    
                    //println!(
                    //    "Tile\tx pos: {},\ty pos: {}", 
                    //    offset_x + x_coord as f32, 
                    //    offset_y + y_coord as f32
                    //);
                    
                    tile_transform.set_translation_xyz(
                        offset_x + x_coord as f32,
                        offset_y + y_coord as f32,
                        1.0 - (l as f32 * 0.01)
                    );
                    // Stop gaps between sprites
                    tile_transform.set_scale(
                        Vector3::new(1.001, 1.001, 1.0)
                    );

                    if tile.flip_v {
                        tile_transform.append_rotation_x_axis(
                            std::f32::consts::PI
                        );
                    }
                    if tile.flip_h {
                        tile_transform.append_rotation_y_axis(
                            std::f32::consts::PI
                        );
                    }
                    if tile.flip_d {
                        tile_transform.set_rotation_z_axis(
                            std::f32::consts::PI/2.0
                        );
                        if tile.flip_h {
                            tile_transform.append_rotation_x_axis(
                                std::f32::consts::PI
                            );
                            if !tile.flip_v {
                                tile_transform.append_rotation_y_axis(
                                    std::f32::consts::PI
                                );
                            }
                        }
                    }

                    if animated_sprite_data.contains_key(&tile.gid) {
                        world
                        .create_entity()
                        .with(tile_transform)
                        .with(tile_sprite)
                        .with(animated_sprite_data[&tile.gid].clone())
                        .build();
                    } else {
                        world
                        .create_entity()
                        .with(tile_transform)
                        .with(tile_sprite)
                        .build();
                    }
                }
            }
        }
        self.map = Some(map);
    }
}

fn init_camera(world: &mut World) {
    let (width, height) = {
        let dimensions = world.read_resource::<ScreenDimensions>();
        (dimensions.width(), dimensions.height())
    };

    let mut trans = Transform::default();

    trans.set_translation_xyz(width * 0.5, height * 0.5, 10.0);


    world
        .create_entity()
        .with(trans)
        .with(Camera::standard_2d(width, height))
        //.with(CameraOrtho::normalized(CameraNormalizeMode::Contain))
        .named("main_camera")
        .build();
}

fn load_texture(path: String, world: &World) -> Handle<Texture>{
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        path,
        ImageFormat::default(),
        (),
        &texture_storage
    )
}

