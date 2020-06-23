use amethyst::{
    assets::{Asset, AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, VecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
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

fn Load_map_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let jap_city_texture_handle = loader.load(
        "../../Map/japanesecitygameassets_windows/RPGMakerVXAce/GK_JC_A5_2.png",
        ImageFormat::default(),
        (),
        &texture_storage
    );
    let jap_city_decor_texture_handle = loader.load(
        "../../Map/japanesecitygameassets_windows/RPGMakerVXAce/GK_JC_B_2.png",
        ImageFormat::default(),
        (),
        &texture_storage
    );


}

mod map {
    use amethyst::{
        assets::{Asset, Handle},
        core::transform::Transform,
        ecs::{VecStorage, prelude::World},
        renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    };

    pub struct Property {
        pub name: String,
        pub value: usize,
    }

    pub struct Object {
        pub width: f32,
        pub height: f32,
        pub name: String,
        pub rotation: f32,
        pub visible: bool,
        pub x: f32,
        pub y: f32,
        pub properties: Option<Vec<Property>>,
    }

    pub struct Layer {
        pub name:       String,
        pub opacity:    f32,
        pub visible:    bool,
        pub x:          f32,
        pub y:          f32,
        pub objects:    Vec<Object>
    }

    pub struct Map {
        pub width:      u32,
        pub height:     u32,
        pub tilewidth:  u32,
        pub tileheight: u32,
        pub layers:     Vec<Layer>
    }

    impl Asset for Map {
        const NAME: &'static str = "epidemic::Map";
        type Data = Self;
        type HandleStorage = VecStorage<Handle<Map>>;
    }

    impl Map {
        pub fn load_layers(&self, world: &mut World) {
            for layer in self.layers.iter() {
                self.load_layer(world, layer);
            }
        }

        fn load_layer(&self, world: &mut World, layer: &Layer) {

            for  obj in layer.objects.iter() {
                let mut transform = Transform::default();

                transform.set_translation_xyz(
                    x: f32, 
                    y: f32, 
                    z: f32
                );
            }
        }
    }

    fn load_sprite_sheet(worl: &mut World) {

    }

    fn load_sprite_sheet_helper
        (world: &mut World, png_path: &str, ron_path: &str) {
            
    }
}