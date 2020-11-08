
use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{
        transform::Transform,
        math::{Vector3, Vector2, Point3},
    },
    prelude::*,
    ecs::prelude::{Entity, Component, DenseVecStorage},
    renderer::{
        sprite::{SpriteRender, Sprite, SpriteSheet},
        Camera, ImageFormat, Texture},
    window::{ScreenDimensions}
};

use std::{
    collections::HashMap,
    io::BufReader,
    path::Path,
    fs::File
    };

use rand::Rng;

use crate::states::game::{
    GameState,
    PlayStateEnum
};
use crate::infection;
use crate::tools;
use crate::systems::animation::AnimatedSprite;

pub struct GameStateBuilder {
    map:                Option<tiled::Map>,
    houses:             Vec<Entity>,
    workplaces:         Vec<Entity>,
    people:             Vec<Entity>,
    camera:             Option<Entity>,
    play_state:         PlayStateEnum,
    path_planner:       Option<tools::path_planner::PathPlanner>,
    male_spritesheet:   Option<Handle<SpriteSheet>>,
    female_spritesheet: Option<Handle<SpriteSheet>>
}

fn get_tiled_tileset_from_file(path: &str, world: &mut World) -> tiled::Tileset {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();

    let file = File::open(path);
    let reader = BufReader::new(
        file.expect(&format!("Failed to get file on path: {}", path))
    );
    
    tiled::parse_tileset(reader, 0).expect(&format!("Failed to parse tileset file on path: {}", path))
}

fn extract_sprites_from_tileset(tileset: &tiled::Tileset, world: &mut World) -> Handle<SpriteSheet> {
    let image = &tileset.images[0];
    let mut sprite_vec = Vec::<Sprite>::new();

    let column_count = image.width as u32/tileset.tile_width;
    let row_count = image.height as u32/tileset.tile_height;

    for column in 0..column_count {
        for row in 0..row_count {
            let sprite = Sprite::from_pixel_values(
                image.width as u32,
                image.height as u32,
                tileset.tile_width,
                tileset.tile_height,
                column*tileset.tile_width,
                row*tileset.tile_height,
                [0.0, -4.0],
                false, 
                false
            );

            sprite_vec.push(sprite);
        }
    }

    let spritesheet = SpriteSheet {
        texture:    load_texture(["../../Map".to_owned(), image.source.clone()].join("/"), world),
        sprites:    sprite_vec
    };

    world.read_resource::<Loader>().load_from_data(
        spritesheet,
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>()
    )
}

impl Default for GameStateBuilder {
    fn default() -> Self {
        GameStateBuilder {
            map:                None,
            houses:             Vec::new(),
            workplaces:         Vec::new(),
            people:             Vec::new(),
            camera:             None,
            play_state:         PlayStateEnum::InGame,
            path_planner:       None,
            male_spritesheet:   None,
            female_spritesheet: None
        }
    }
}

impl GameStateBuilder {

    pub fn build(self) -> GameState {
        println!(
            "Unwrapping game state! Map: {}, Camera: {}, PathPlanner: {}",
            self.map.is_some(),
            self.camera.is_some(),
            self.path_planner.is_some()
        );

        GameState::new(
            self.map.unwrap(),
            self.houses,
            self.workplaces,
            self.people,
            self.camera.unwrap(),
            self.play_state,
            self.path_planner.unwrap()
        )
    }

    pub fn init_camera(&mut self, world: &mut World) {
        let (width, height) = {
            let dimensions = world.read_resource::<ScreenDimensions>();
            (dimensions.width(), dimensions.height())
        };
    
        let mut trans = Transform::default();
    
        trans.set_translation_xyz(width * 0.5, height * 0.5, 10.0);
    
        let cam =world
            .create_entity()
            .with(trans)
            .with(Camera::standard_2d(width, height))
            .named("main_camera")
            .build();
        
        self.camera = Some(cam);
    }

    fn load_people_sprites(
        &mut self,
        // tsx_rel_path: &str,
        world: &mut World
    ) {
        let male_sheet = get_tiled_tileset_from_file("../Map/male_sprites.tsx", world);
        let female_sheet = get_tiled_tileset_from_file("../Map/female_sprites.tsx", world);

        self.male_spritesheet = Some(
            extract_sprites_from_tileset(
                &male_sheet,
                world
            )
        );
        self.female_spritesheet = Some(
            extract_sprites_from_tileset(
                &female_sheet,
                world
            )
        );
    }

    fn load_map(
        &mut self,
        map_path: &str, 
        image_rel_path: &str, 
        world: &mut World) {
        // Get texture handle for the tileset image
        self.load_people_sprites(world);
        
        // Load the tiled map
        let file = File::open(&Path::new(map_path)).unwrap();
        let reader = BufReader::new(file);
        let map = tiled::parse(reader).unwrap();
    
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
            let layer_tiles: Vec<Vec<tiled::LayerTile>>;
            // Only load Finite data
            if let tiled::LayerData::Finite(data) = &layer.tiles {
                layer_tiles = data.clone();
            } else {
                continue;
            }
            for (y, row) in layer_tiles.iter().rev().enumerate().clone() {
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

    pub fn load_game_map(&mut self, world: &mut World) {
        world.register::<infection::buildings::Building>();
        world.register::<infection::buildings::BuildingEntrance>();
        world.register::<infection::buildings::MaxOccupants>();
        world.register::<infection::buildings::Location>();
        world.register::<infection::population::Person>();
        world.register::<infection::population::Occupants>();
        world.register::<infection::population::Residence>();
        world.register::<infection::population::Job>();

        self.load_map(
            "../Map/MainTown.tmx",
            "../../Map",
            world
        );

        if let Some(map) = &self.map {
            // Create new Path planner map (give it the new size)
            // Load all elements of map (building locations, people counts)
            let map_size = (map.width as u64 * 32, map.height as u64 * 32);
            let mut people_count = 0;
            let mut house_entrance: Vec<infection::buildings::BuildingEntrance> = Vec::new();
            let mut work_entrance: Vec<infection::buildings::BuildingEntrance> = Vec::new();
            
            let mut planner = tools::path_planner::PathPlanner::new((map.width as usize, map.height as usize));

            for object_group in &map.object_groups {
                if object_group.name == "Homes" {
                    for home_object in &object_group.objects {
                        match home_object.shape {
                            tiled::ObjectShape::Rect { width, height } => {
                                let people_count_prop = home_object.properties
                                    .get("peopleCount")
                                    .expect("No peopleCount variable found!");

                                if let tiled::PropertyValue::IntValue(int_val) = people_count_prop {
                                    people_count += int_val;
                                } else {
                                    panic!("Failed on getting person count!");
                                }

                                let home = {
                                    let max_occupant_count: usize;
                                    let prop_val = home_object
                                                                .properties
                                                                .get("peopleCount")
                                                                .expect("Object did not have 'peopleCount' property!");
                                    
                                    if let tiled::PropertyValue::IntValue(int_val) = prop_val {
                                        max_occupant_count = *int_val as usize;
                                    } else {
                                        println!("Failed to find 'peopleCount' integer, getting random number!");
                                        max_occupant_count =  rand::thread_rng().gen_range(3, 25);
                                    }

                                    println!("New home location: {:?}, map size: {:?}", (home_object.x, home_object.y), map_size);

                                    let size = [(width/32.0).round(), (height/32.0).round()];

                                    let location = infection::buildings::Location::new(
                                        (home_object.x/32.0).round(),
                                        ((map_size.1 as f32 - home_object.y)/32.0).round() - size[1]);

                                    let max_occupants = infection::buildings::MaxOccupants::new(
                                        max_occupant_count
                                    );

                                    let building = infection::buildings::Building::new(home_object.id, size);

                                    world.create_entity()
                                        .with(building)
                                        .with(location)
                                        .with(max_occupants)
                                        .build()
                                };

                                self.houses.push(
                                    home.clone()
                                );
                                
                                let max_occupants = world
                                    .read_component::<infection::buildings::MaxOccupants>()
                                    .get(home)
                                    .expect("Failed to get building component")
                                    .get_max_occupants();

                                let mut occupants = infection::population::Occupants::new();

                                for _ in 0..max_occupants {
                                    // let new_person = infection::population::Person::new_with_residence(
                                    //     home.clone(),
                                    //     world
                                    // );
                                    let person = infection::population::Person::new_person();
                                    let residence = infection::population::Residence::new(home.clone());

                                    let sprite_render = SpriteRender {
                                        sprite_sheet:   self.female_spritesheet.clone().unwrap(),
                                        sprite_number:  1
                                    };

                                    let mut transform = Transform::default();

                                    transform.set_translation_xyz(
                                        32.0/2.0,
                                        32.0/2.0,
                                        1.1
                                    );
                                

                                    let person_ent = world
                                        .create_entity()
                                        .with(person)
                                        .with(residence)
                                        .with(sprite_render)
                                        .with(transform)
                                        .build();

                                    self.people.push(person_ent);
                                    occupants.add(person_ent.clone());
                                }

                                let res = world
                                    .write_storage::<infection::population::Occupants>()
                                    .insert(home, occupants);
                                
                                if let Err(er) = res {
                                    println!("Failed to add Occupants to buildings! Error: {}", er);
                                }
                            }
                            tiled::ObjectShape::Point(x, y) => {
                                let location = infection::buildings::Location::new(
                                    (x/32.0).round(), 
                                    map.height as f32 - (y/32.0).round()
                                );

                                house_entrance.push(
                                    infection::buildings::BuildingEntrance::new(location)
                                );
                            }
                            _ => println!("Unknown shape for home building!")
                        }
                    }
                }

                if object_group.name == "WorkPlaces" {
                    for work_building in &object_group.objects {
                        match work_building.shape {
                            tiled::ObjectShape::Rect {width, height} => {
                                let building_ent  = {

                                    let size = [
                                        (width/32.0).round(), 
                                        (height/32.0).round()
                                    ];
        
                                    let location = infection::buildings::Location::new(
                                        (work_building.x/32.0).round(), 
                                        ((map_size.1 as f32 - work_building.y)/32.0).round() - size[1]
                                    );
                                    
                                    let building = infection::buildings::Building::new(
                                        work_building.id, 
                                        size
                                    );
        
                                    world.create_entity()
                                        .with(building)
                                        .with(location)
                                        .build()
                                };

                                self.workplaces.push(building_ent);
                            }
                            tiled::ObjectShape::Point(x, y) => {
                                let location = infection::buildings::Location::new(
                                    (x/32.0).round(), 
                                    map.height as f32 - (y/32.0).round()
                                );

                                work_entrance.push(
                                    infection::buildings::BuildingEntrance::new(location)
                                );
                            }
                            _ => println!("Unrecognised map shape!")
                        }
                    }
                }

                if object_group.name == "Walking_Path" {
                    for walking_path in &object_group.objects {
                        let loc = (
                            (walking_path.x/32.0).floor() as u32, 
                            (walking_path.y/32.0).floor() as u32
                        );
                        let size = (
                            (walking_path.width/32.0).ceil() as u32, 
                            (walking_path.height/32.0).ceil() as u32
                        );
                        
                        planner.add_path_blocks(loc, size);
                    }
                }
            }
            // Completed loading information into path planner
            self.path_planner = Some(planner);

            // Start loading entrances on buildings
            println!("Got {} building entrances for {} workplaces", 
                work_entrance.len(), 
                self.workplaces.len()
            );
            for entrance in work_entrance {
                for building_ent in &self.workplaces {
                    let location_getter = world
                        .read_component::<infection::buildings::Location>();
                    let building_getter = world
                        .read_component::<infection::buildings::Building>();
                    
                    let building_location = location_getter.get(*building_ent)
                        .expect("Failed to get buiilding location!");
                    let building = building_getter.get(*building_ent)
                        .expect("Failed to get buiilding!");
                    
                    // If location of entrance is not i less (on Y) than building, go to next building
                    if &entrance.location.y() == &(building_location.y() - 1.0) {
                        
                        if (&entrance.location.x() >= &building_location.x()) && 
                           (&entrance.location.x() < &(building_location.x()+building.size[0])) {
                            let insert = world
                                .write_component::<infection::buildings::BuildingEntrance>()
                                .insert(*building_ent, entrance.clone());
                            
                            if let Err(er) = insert {
                                println!("Failed to add entrance to building! Error: {}", er);
                            }
                        }
                    }
                }
            }
            println!("Got {} building entrances for {} houses", 
                house_entrance.len(), 
                self.houses.len()
            );
            for entrance in house_entrance {
                for building_ent in &self.houses {
                    let location_getter = world
                        .read_component::<infection::buildings::Location>();
                    let building_getter = world
                        .read_component::<infection::buildings::Building>();
                    
                    let building_location = location_getter.get(*building_ent)
                        .expect("Failed to get buiilding location!");
                    let building = building_getter.get(*building_ent)
                        .expect("Failed to get buiilding!");
                    
                    // If location of entrance is not i less (on Y) than building, go to next building
                    if &entrance.location.y() == &(building_location.y() - 1.0) {
                        
                        if (&entrance.location.x() >= &building_location.x()) && 
                           (&entrance.location.x() < &(building_location.x()+building.size[0])) {
                            let insert = world
                                .write_component::<infection::buildings::BuildingEntrance>()
                                .insert(*building_ent, entrance.clone());
                            
                            if let Err(er) = insert {
                                println!("Failed to add entrance to building! Error: {}", er);
                            }
                        }
                    }
                }
            }
            println!("Created {} people!", self.people.len());
            println!("Loading jobs for people!");

            for person in &self.people {
                let result = world
                    .write_component::<infection::population::Job>()
                    .insert(*person, infection::population::Job::new(self.workplaces[0]));

                if let Err(er) = result {
                    println!("Error occured during job allocation: {}", er);
                }
            }
        }
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
