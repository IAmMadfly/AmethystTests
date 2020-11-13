use std::time;

use amethyst::{
    prelude::*,
    ecs::{Entity, Component, DenseVecStorage, DefaultVecStorage},
    core::transform::Transform,
    renderer::SpriteRender
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
    fn new_person() -> Person {
        Person {
            name:       Person::generate_random_name(),
            sex:        rand_sex(),
            age:        rand::random(),
            infection:  None
        }
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

        let person_in_home = InBuilding {
            building:   residence_ent,
            start_time: time::Duration::new(0, 0)
        };


        world.create_entity()
            .with(person)
            .with(residence)
            .with(person_in_home)
            .build()
    }

    fn generate_random_name() -> String {
        let mut gen = Generator::with_naming(Name::Plain);
        gen.next().unwrap()
    }
}

pub struct PersonEntBuilder {
    person:     Person,
    residence:  Residence,
    sprite:     SpriteRender,
    job:        Option<Job>
}

impl PersonEntBuilder {
    pub fn new(residence: Residence, sprite: SpriteRender) -> Self {
        PersonEntBuilder {
            person:     Person::new_person(),
            residence,
            sprite,
            job:    None
        }
    }

    pub fn add_job(&mut self, job: Job) {
        self.job = Some(job);
    }

    pub fn build(self, world: &mut World) -> Entity {
        let mut person_ent_builder = world
            .create_entity()
            .with(self.person)
            .with(self.residence)
            .with(self.sprite);

        if let Some(job) = self.job {
            person_ent_builder = person_ent_builder.with(job);
        }

        person_ent_builder.build()
    }
}


pub struct Residence {
    home:       Entity
}

impl Component for Residence {
    type Storage = DenseVecStorage<Self>;
}

impl Residence {
    pub fn new(home: Entity) -> Residence {
        Residence {
            home
        }
    }
}

pub struct Job {
    building:       Entity
}

impl Component for Job {
    type Storage = DenseVecStorage<Self>;
}

impl Job {
    pub fn new(building: Entity) -> Job {
        Job {
            building
        }
    }
}

pub struct InBuilding {
    building:       Entity,
    start_time:     time::Duration
}

impl Component for InBuilding {
    type Storage = DenseVecStorage<Self>;
}

pub struct Traveling {
    location:       buildings::Location,
    path_plan:      Vec<(u16, u16)>
}

impl Component for Traveling {
    type Storage = DenseVecStorage<Self>;
}