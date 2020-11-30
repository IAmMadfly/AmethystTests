use amethyst::{
    prelude::*,
    ecs::{Entity, EntityBuilder, Component, DenseVecStorage},
    renderer::SpriteRender,
    core::Transform
};

use names::{Generator, Name};
use crate::infection::infection;
use crate::infection::buildings;

use time::{
    time,
    Time,
    Weekday,
    PrimitiveDateTime,
    Duration
};

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

// pub struct Occupants {
//     people:         Vec<Entity>
// }

// impl Component for Occupants {
//     type Storage    = DenseVecStorage<Self>;
// }

// impl Occupants {
//     pub fn new() -> Self {
//         Occupants {
//             people:     Vec::new()
//         }
//     }

//     pub fn add(&mut self, person: Entity) {
//         self.people.push(person);
//     }
// }

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
            start_time: (*world.read_resource::<PrimitiveDateTime>()).clone()
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
    person:         Person,
    residence:      Residence,
    in_building:    InBuilding,
    sprite:         SpriteRender,
    transform:      Transform,
    job:            Option<Job>
}

impl PersonEntBuilder {
    pub fn new(
        residence: Residence, 
        sprite: SpriteRender, 
        world: &mut World
    ) -> PersonEntBuilder {

        let transform = Transform::default();

        let start_time = world.read_resource::<PrimitiveDateTime>().clone();

        let in_building = InBuilding{
            start_time:     *start_time.clone(),
            building:       residence.home.clone()
        };

        PersonEntBuilder {
            person:     Person::new_person(),
            residence,
            in_building:    in_building,
            sprite,
            job:        None,
            transform
        }
    }

    pub fn add_job(&mut self, job: Job) {
        self.job = Some(job);
    }

    pub fn get_entity_builder(self, world: &mut World) -> EntityBuilder {
        let mut builder = world.create_entity()
            .with(self.person)
            .with(self.residence)
            .with(self.sprite);

        if let Some(job) = self.job {
            builder = builder.with(job);
        }

        return builder
    }

    pub fn build(self, world: &mut World) -> Entity {
        let mut person_ent_builder = world
            .create_entity()
            .with(self.person)
            .with(self.residence)
            .with(self.sprite);

        if let Some(job) = self.job {
            person_ent_builder = person_ent_builder.with(job);
            println!("Made person with job!");
        } else {
            println!("Made person without job!");
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

fn get_weekday_index(weekday: Weekday) -> usize {
    match weekday {
        Weekday::Monday => 0,
        Weekday::Tuesday => 1,
        Weekday::Wednesday => 2,
        Weekday::Thursday => 3,
        Weekday::Friday => 4,
        Weekday::Saturday => 5,
        Weekday::Sunday => 6,
    }
}

pub struct Job {
    building:       Entity,
    work_time:      [Option<(time::Time, Duration)>; 7]
}

impl Component for Job {
    type Storage = DenseVecStorage<Self>;
}

impl Job {
    pub fn new(building: Entity) -> Job {
        Job {
            building,
            work_time:      [
                Some((time!(9:00 am), Duration::hours(8))),
                Some((time!(9:00 am), Duration::hours(8))),
                Some((time!(9:00 am), Duration::hours(8))),
                Some((time!(9:00 am), Duration::hours(8))),
                Some((time!(9:00 am), Duration::hours(8))),
                None,
                None
            ]
        }
    }

    pub fn work_started(&self, datetime: PrimitiveDateTime) -> bool {
        let index = get_weekday_index(datetime.weekday());

        if let Some((Time, Duration)) = self.work_time[index] {
            return true
        } else {
            return false
        }
    }
}

pub struct InBuilding {
    building:       Entity,
    start_time:     time::PrimitiveDateTime
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