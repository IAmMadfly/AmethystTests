use std::rc::{Rc, Weak};

use amethyst::{
    prelude::*,
    ecs::{Entity, Component, DenseVecStorage, DefaultVecStorage}
};

use num::traits::Num;
use tiled;
use rand::Rng;

use crate::infection::infection;

#[derive(Debug)]
enum HomeType {
    Appartment,
    _House,
    _Farm
}

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

pub struct Size {
    x:      f32,
    y:      f32
}

impl Component for Size {
    type Storage = DenseVecStorage<Self>;
}

impl Size {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
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

pub struct Home {
    id:                 u32,
    home_type:          HomeType,
    families:           Vec<Entity>,
    //pub location:           (u32, u32),
    //pub size:               (u32, u32)
}

impl Component for Home {
    type Storage = DenseVecStorage<Self>;
}

impl Home {
    pub fn families(&self) -> &Vec<Entity> {
        &self.families
    }
}

enum FamilyType {
    Single,
    Partner,
    Children
}

pub struct Family {
    parents:                Option<[Entity; 2]>,
    children:               Vec<Entity>,
    fam_type:               FamilyType
}

impl Component for Family {
    type Storage = DenseVecStorage<Self>;
}

impl Family {
    fn generate_families(family_count: u32, world: &mut World) -> Vec<Entity> {
        // Should be a vector of Familiy Entities
        let vector: Vec<Entity> = Vec::new();
        println!("{} people", family_count);

        let mut families_made = 0;

        while (family_count-families_made) > 0 {
            let random_float: f32 = rand::random();

            if random_float < 0.2 {
                vector.push(
                    world.create_entity()
                        .with(Family::make(FamilyType::Single, world))
                        .build()
                );
                families_made += 1;
            } else if random_float < 0.4 {

                families_made += 1;
            } else if random_float < 0.8 {

                families_made += 1;
            } else if random_float < 1.0 {

                families_made += 1;
            };
        }
        vector
    }

    fn make(fam_type: FamilyType, world: &mut World) -> Family {
        match fam_type {
            FamilyType::Single => {
                return Family {
                    parents:    None,
                    fam_type:   fam_type,
                    children:   Vec::<Entity>::new()
                }
            }
            FamilyType::Partner => {
                return Family {
                    parents:    None,
                    fam_type:   fam_type,
                    children:   Vec::<Entity>::new()
                }
            }
            FamilyType::Children => {
                return Family {
                    parents:    None,
                    fam_type:   fam_type,
                    children:   Family::make_rand_children(world)
                }
            }
        }
    }

    fn make_rand_children(world: &mut world) {

    }

}

enum Relationship {
    Partner(Entity),
    Parent(Entity),
    Child(Entity)
}

struct Person {
    family_id:      u64,
    name:           String,
    sex:            Sex,
    infection:      Option<infection::Disease>,
    relationships:  Vec<Relationship>
}

impl Component for Person {
    type Storage = DenseVecStorage<Self>;
}

impl Home {
    pub fn new(home_data: &tiled::Object, map_size: (u64, u64), world: &mut World) -> Entity {
        let mut family_count =  0;
        let prop_val =          home_data
                                    .properties
                                    .get("peopleCount")
                                    .expect("Object did not have 'peopleCount' property!");
        
        if let tiled::PropertyValue::IntValue(int_val) = prop_val {
            family_count = *int_val as u32;
        } else {
            println!("Failed to find 'peopleCount' integer, getting random number!");
            family_count =  rand::thread_rng().gen_range(3, 25);
        }

        println!("New home location: {:?}, map size: {:?}", (home_data.x, home_data.y), map_size);

        let size = Size {
            x:  (home_data.width/32.0).round(), 
            y:  (home_data.height/32.0).round()
        };

        let location = Location {
            x:  (home_data.x/32.0).round(), 
            y:  ((map_size.1 as f32 - home_data.y)/32.0).round() - size.y
        };

        let home = Home {
            id:             home_data.id,
            home_type:      HomeType::Appartment,
            families:       Family::generate_families(family_count, world)
            //size:           size,
            //location:       ((home_data.x/32.0).round() as u32, (((map_size.1 as f32 - home_data.y)/32.0).round() as u32 - size.1))
        };

        world.create_entity()
            .with(home)
            .with(location)
            .with(size)
            .build()
    }
}