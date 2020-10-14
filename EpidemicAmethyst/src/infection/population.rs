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
    _id:                u32,
    pub max_occupants:  usize,
    pub size:           [f32; 2]
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

impl Occupants {
    pub fn new() -> Self {
        Occupants {
            people:     Vec::new()
        }
    }

    pub fn add(&mut self, person: Entity) {
        self.people.push(person);
    }
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
        let person = Person {
            name:       Person::generate_random_name(),
            sex:        rand_sex(),
            age:        rand::random(),
            infection:  None
        };

        world.create_entity()
            .with(person)
            .build()
    }

    pub fn new_with_residence(residence_ent: Entity, world: &mut World) -> Entity {
        let person = Person {
            name:       Person::generate_random_name(),
            sex:        rand_sex(),
            age:        rand::random(),
            infection:  None
        };

        let residence = Residence {
            home:       residence_ent
        };

        world.create_entity()
            .with(person)
            .with(residence)
            .build()
    }

    fn generate_random_name() -> String {
        let mut gen = Generator::with_naming(Name::Plain);
        gen.next().unwrap()
    }
}

pub struct Residence {
    home:       Entity
}

impl Component for Residence {
    type Storage = DenseVecStorage<Self>;
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