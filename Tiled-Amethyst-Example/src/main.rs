#![allow(dead_code, unused_imports)]

extern crate amethyst;

mod lib;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::{Transform, TransformBundle},
    ecs::prelude::Entity,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        Camera,
        ImageFormat,
        plugins::{RenderFlat2D, RenderToWindow},
        RenderingBundle,
        rendy::hal::command::ClearColor,
        Sprite, sprite::TextureCoordinates, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, types::DefaultBackend,
    },
    tiles::{FlatEncoder, MortonEncoder2D, RenderTiles2D},
    utils::application_root_dir,
    window::{DisplayConfig, ScreenDimensions, Window},
};
use lib::parse;


pub fn initialize_camera(world: &mut World) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    
    let mut transform = Transform::default();
    // Camera z = 10.0 is usually a good starting point
    transform.set_translation_xyz(width * 0.5, height * 0.5, 10.0);
    
    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build()
}


struct GameplayState;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
    
        // We need the camera to actually see anything
        initialize_camera(world);
        
        // Load the tiled map the "crude" way
        load_map(world);
        println!("Finished loading map!");
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit
            }
        }
        Trans::None
    }

    fn update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);
        Trans::None
    }
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

fn load_map(world: &mut World) {
    // Get texture handle for the tileset image
    let texture_handle = load_texture("assets/terrainTiles_default.png".to_owned(), world);


    
    
    // Load the tiled map
    let file = File::open(&Path::new("resources/assets/tiled_base64_zlib.tmx")).unwrap();
    let reader = BufReader::new(file);
    let map = parse(reader).unwrap();

    let first_tileset = map.tilesets.get(0).expect("No tilesets found!");
    
    let first_image: &lib::Image = {
        let image_vec = &first_tileset.images;
        if image_vec.len() > 0 {
            image_vec.get(0).expect("Failed to get first image in tileset")
        } else {
            let tiles = &first_tileset.tiles;
            tiles.get(0).expect("Failed to get first tile").images.get(0).expect("Failed to get first image in tile")
        }
    };

    let tile_width = first_tileset.tile_width as i32;
    let tile_height = first_tileset.tile_height as i32;
    let tileset_width = first_image.width;
    let tileset_height = first_image.height;
    let tileset_sprite_columns = tileset_width / tile_width as i32;
    let tileset_sprite_rows = tileset_height / tile_height as i32;

    let mut sprite_sheet_handles: HashMap<u32, Handle<SpriteSheet>> = HashMap::new();
    for tileset in map.tilesets.iter() {
        
        for image in tileset.images.iter() {

            let texture_hand = load_texture(
                ["assets".to_owned(), image.source.clone()].join("/"), 
                world
            );
            let mut sprites: Vec<Sprite> = Vec::new();

            let tile_width = first_tileset.tile_width as i32;
            let tile_height = first_tileset.tile_height as i32;
            let tileset_width = first_image.width;
            let tileset_height = first_image.height;
            let tileset_sprite_columns = tileset_width / tile_width as i32;
            let tileset_sprite_rows = tileset_height / tile_height as i32;
            for x in 0..tileset_sprite_columns {
                for y in 0..tileset_sprite_rows {
                    let tileset_w = *&tileset.images[0].width as u32;
                    let tileset_h = *&tileset.images[0].height as u32;
                    let sprite_w = tile_width as u32;
                    let sprite_h = tile_height as u32;
                    let offset_x = (y * tile_width) as u32;
                    let offset_y = (x * tile_height) as u32;
                    let offsets = [0.0; 2];
                    // Create a new `Sprite`
                    let sprite = Sprite::from_pixel_values(
                        tileset_w,
                        tileset_h,
                        sprite_w,
                        sprite_h,
                        offset_x,
                        offset_y,
                        offsets,
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
                let loader = world.read_resource::<Loader>();
                let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
                
                loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
            };
            sprite_sheet_handles.insert(
                tileset.first_gid,
                sprite_sheet_handle
            );
            println!("{}: {}", tileset.first_gid, ["assets".to_owned(), image.source.clone()].join("/"));
        }

        for tile in tileset.tiles.iter() {
            for image in tile.images.iter() {

                let texture_hand = load_texture(
                    ["assets".to_owned(), image.source.clone()].join("/"), 
                    world
                );

                let mut sprites: Vec<Sprite> = Vec::new();
                let tileset_w = first_image.width as u32;
                let tileset_h = first_image.height as u32;
                let sprite_w = tile_width as u32;
                let sprite_h = tile_height as u32;
                let offset_x = 0 as u32;
                let offset_y = 0 as u32;
                let offsets = [0.0; 2];
                // Create a new `Sprite`
                let sprite = Sprite::from_pixel_values(
                    tileset_w,
                    tileset_h,
                    sprite_w,
                    sprite_h,
                    offset_x,
                    offset_y,
                    offsets,
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
                println!("{}: {}", tileset.first_gid + tile.id, ["assets".to_owned(), image.source.clone()].join("/"));
            }
        }
    }
    
    if let Some(map_tileset) = map.get_tileset_by_gid(1) {
        
        
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
            println!("Layer: {}", l);
            if !layer.visible {
                continue
            }
            for (y, row) in layer.tiles.iter().rev().enumerate().clone() {
                for (x, &tile) in row.iter().enumerate() {
                    // Do nothing with empty tiles
                    if tile.gid == 0 {
                        continue;
                    }
                    //println!("-- Start --");
                    
                    // Tile ids start from 1 but tileset sprites start from 0
                    let tile_id = tile.gid - 1;
                    let sprite_sheet_hash = {
                        let mut num = sprite_sheet_keys.get(sprite_sheet_keys.len() - 1).expect("Failed to get last element in vec").clone();
                        for sprite_sheet_key in sprite_sheet_keys.iter().rev() {
                            println!("Curr spt_hash: {}, tile.gid: {}", sprite_sheet_key, tile.gid);
                            if sprite_sheet_key <= &tile.gid {
                                println!("-- Chosen! --");
                                num = sprite_sheet_key.clone();
                                break;
                            }
                        }
                        num
                    };
                    println!("Tile.gid: {}, Sprite hash: {}", tile.gid, sprite_sheet_hash);

                    //println!("Stage 1");
                    // Sprite for the tile
                    let tile_sprite = SpriteRender {
                        sprite_sheet: sprite_sheet_handles.get(&sprite_sheet_hash).expect("Got unexpected hash!").clone(),  //sprite_sheet_handle.clone(),
                        sprite_number: (tile.gid - 1) as usize,
                    };
                    //println!("Stage 2");
                    
                    // Where we should draw the tile?
                    let mut tile_transform = Transform::default();
                    let x_coord = x * tile_width as usize;
                    let y_coord = (y as f32 * tile_height as f32) + tile_height as f32;
                    // Offset the positions by half the tile size so they're nice and snuggly on the screen
                    // Alternatively could use the Sprite offsets instead: [-32.0, 32.0]. Depends on the use case I guess.
                    let offset_x = tile_width as f32/2.0;
                    let offset_y = -tile_height as f32/2.0;
                    //println!("Stage 3");
                    
                    tile_transform.set_translation_xyz(
                        offset_x + x_coord as f32,
                        offset_y + y_coord as f32,
                        1.0
                    );
                    //println!("Stage 4");
                    
                    
                    // Create the tile entity
                    world
                        .create_entity()
                        .with(tile_transform)
                        .with(tile_sprite)
                        .build();
                    //println!("-- End --");
                }
            }
        }
    } else {
        println!("Failed to print map!!");
    }
}

fn main() -> Result<(), amethyst::Error> {
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Warn)
        .start();

    let app_root = application_root_dir()?;
    let resources = app_root.join("resources");
    let display_config_path = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
        )?;
    
    let mut game = Application::build(resources, GameplayState)?.build(game_data)?;
    
    game.run();
    Ok(())
}
