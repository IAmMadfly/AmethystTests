#![allow(dead_code, unused_imports)]

extern crate amethyst;

extern crate tiled;
//mod lib;

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
//use lib::parse;
use tiled::parse;

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
        //let _map = load_map(
        //    "resources/assets/tiled_base64_zlib.tmx",
        //    "assets",
        //    world
        //);
        let map = load_map(
            "../Map/MainTown.tmx",
            "../../Map",
            world
        );
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

fn load_map(
    map_path: &str, 
    image_rel_path: &str, 
    world: &mut World) -> tiled::Map {
    // Get texture handle for the tileset image
    //let texture_handle = load_texture("assets/terrainTiles_default.png".to_owned(), world);
    
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

    let mut sprite_sheet_handles: HashMap<u32, Handle<SpriteSheet>> = HashMap::new();
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
            //println!("{}", [image_rel_path.to_owned(), image.source.clone()].join("/"));
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
                let loader = world.read_resource::<Loader>();
                let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
                
                loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
            };
            sprite_sheet_handles.insert(
                tileset.first_gid,
                sprite_sheet_handle
            );
            //println!(
            //    "Tileset first_gid:{}, Image: {}", 
            //    tileset.first_gid, [image_rel_path.to_owned(), image.source.clone()].join("/")
            //);
        }

        for tile in tileset.tiles.iter() {
            for image in tile.images.iter() {

                let texture_hand = load_texture(
                    [image_rel_path.to_owned(), image.source.clone()].join("/"), 
                    world
                );
                //println!("{}", [image_rel_path.to_owned(), image.source.clone()].join("/"));

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
                //println!("Tileset first_gid:{}, Image: {}", tileset.first_gid + tile.id, [image_rel_path.to_owned(), image.source.clone()].join("/"));
            }
        }
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
                //println!("Tile.gid: {}, Sprite_hash key: {}", tile.gid, sprite_sheet_hash);
                
                // Sprite for the tile
                let tile_sprite = SpriteRender {
                    sprite_sheet: sprite_sheet_handles.get(&sprite_sheet_hash).expect("Got unexpected hash!").clone(),  //sprite_sheet_handle.clone(),
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
                //println!("Stage 3");
                
                tile_transform.set_translation_xyz(
                    offset_x + x_coord as f32,
                    offset_y + y_coord as f32,
                    1.0 - (l as f32 * 0.1)
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
    map
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
