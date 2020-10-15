use amethyst::{
    prelude::*,
    ecs::{Entity, Component, DenseVecStorage}
};

use rand::Rng;

pub struct Location {
    x:      f32,
    y:      f32
}

impl Component for Location {
    type Storage = DenseVecStorage<Self>;
}

impl Location {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

pub struct Building {
    _id:                u32,
    pub max_occupants:  usize,
    pub size:           [f32; 2]
}

impl Component for Building {
    type Storage    = DenseVecStorage<Self>;
}

impl Building {
    pub fn new(home_data: &tiled::Object, map_size: (u64, u64), world: &mut World) -> Entity {
        let max_occupant_count: usize;
        let prop_val =          home_data
                                    .properties
                                    .get("peopleCount")
                                    .expect("Object did not have 'peopleCount' property!");
        
        if let tiled::PropertyValue::IntValue(int_val) = prop_val {
            max_occupant_count = *int_val as usize;
        } else {
            println!("Failed to find 'peopleCount' integer, getting random number!");
            max_occupant_count =  rand::thread_rng().gen_range(3, 25);
        }

        println!("New home location: {:?}, map size: {:?}", (home_data.x, home_data.y), map_size);

        let size = [(home_data.width/32.0).round(), (home_data.height/32.0).round()];

        let location = Location {
            x:  (home_data.x/32.0).round(), 
            y:  ((map_size.1 as f32 - home_data.y)/32.0).round() - size[1]
        };

        let home = Building {
            _id:            home_data.id,
            max_occupants:  max_occupant_count,
            size:           size
        };

        world.create_entity()
            .with(home)
            .with(location)
            .build()
    }
}