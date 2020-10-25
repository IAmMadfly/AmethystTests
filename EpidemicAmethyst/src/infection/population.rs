use amethyst::{
    prelude::*,
    ecs::{Entity, Component, DenseVecStorage, DefaultVecStorage}
};

use names::{Generator, Name};
use crate::infection::infection;
use crate::infection::buildings;

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

pub struct PathMaker {
    path_blocks:    Vec<buildings::Location>
}