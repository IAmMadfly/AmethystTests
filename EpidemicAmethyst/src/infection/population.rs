use std::rc::{Rc, Weak};

use amethyst::{
    prelude::*,
    ecs::{Entity, Component, DenseVecStorage, DefaultVecStorage}
};

use rand::Rng;
use names::{Generator, Name};
use crate::infection::infection;

#[derive(Debug)]
enum Sex {
    Male,
    Female
}

fn rand_sex() -> Sex {
    if rand::random() {
        Sex::Male
    } else {
        Sex::Female
    }
}

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
    _id:            u32,
    max_occupants:  u32,
    pub size:       [f32; 2]
}

impl Component for Building {
    type Storage    = DenseVecStorage<Self>;
}

pub struct Occupants {
    people:         Vec<Entity>
}

impl Component for Occupants {
    type Storage    = DenseVecStorage<Self>;
}

pub struct Family {
    parents:        Option<[Entity; 2]>,
    children:       Vec<Entity>
}

impl Component for Family {
    type Storage    = DenseVecStorage<Self>;
}

impl Family {
    fn generate_families(max_occupant_count: u32, world: &mut World) -> Vec<Entity> {
        // Should be a vector of Familiy Entities
        Vec::<Entity>::new()
    }

    //fn make(fam_type: FamilyType, world: &mut World) -> Family {
    //    match fam_type {
    //        FamilyType::Single => {
    //            return Family {
    //                parents:    None,
    //                fam_type:   fam_type,
    //                children:   Vec::<Entity>::new()
    //            }
    //        }
    //        FamilyType::Partner => {
    //            return Family {
    //                parents:    None,
    //                fam_type:   fam_type,
    //                children:   Vec::<Entity>::new()
    //            }
    //        }
    //        FamilyType::Children => {
    //            return Family {
    //                parents:    None,
    //                fam_type:   fam_type,
    //                children:   Family::make_rand_children(world)
    //            }
    //        }
    //    }
    //}
}

pub struct Person {
    name:           String,
    sex:            Sex,
    age:            u8,
    infection:      Option<infection::Disease>
}

impl Component for Person {
    type Storage = DenseVecStorage<Self>;
}

impl Person {
    pub fn new(world: &mut World) -> Entity {
        let mut gen = Generator::with_naming(Name::Plain);

        let person = Person {
            name:       gen.next().unwrap(),
            sex:        rand_sex(),
            age:        rand::random(),
            infection:  None
        };

        world.create_entity()
            .with(person)
            .build()
    }
}

impl Building {
    pub fn new(home_data: &tiled::Object, map_size: (u64, u64), world: &mut World) -> Entity {
        let max_occupant_count;
        let prop_val =          home_data
                                    .properties
                                    .get("peopleCount")
                                    .expect("Object did not have 'peopleCount' property!");
        
        if let tiled::PropertyValue::IntValue(int_val) = prop_val {
            max_occupant_count = *int_val as u32;
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

        println!("MAKING NEW BUILDING!");

        world.create_entity()
            .with(home)
            .with(location)
            .build()
    }
}