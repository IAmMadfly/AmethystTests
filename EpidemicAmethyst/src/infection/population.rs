use tiled;
use rand::Rng;
use std::rc::{Rc, Weak};

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

struct Person {
    family_id:      u64,
    name:           String,
    sex:            Sex,
    parents:        [Option<Weak<Person>>; 2],
    children:       Vec<Rc<Person>>,
    infection:      Option<Rc<dyn infection::Disease>>
}

struct Family {
    people:         Vec<Person>
}

impl Family {
    fn generate_families(people_count: u32) -> Vec<Self> {
        let vector: Vec<Family> = Vec::new();
        println!("{} people", people_count);

        vector
    }
}

pub struct Home {
    id:             u32,
    home_type:      HomeType,
    families:       Vec<Family>,
    location:       (u32, u32),
    size:           (u32, u32)
}

impl Home {
    pub fn new(home_data: &tiled::Object) -> Self {
        let mut _people_count =  0;
        let prop_val =          home_data
                                    .properties
                                    .get("peopleCount")
                                    .expect("Object did not have 'peopleCount' property!");
        
        if let tiled::PropertyValue::IntValue(int_val) = prop_val {
            _people_count = *int_val as u32;
        } else {
            println!("Failed to find 'peopleCount' integer, getting random number!");
            _people_count =  rand::thread_rng().gen_range(3, 25);
        }

        Home {
            id:             home_data.id,
            home_type:      HomeType::Appartment,
            families:       Family::generate_families(_people_count),
            location:       (home_data.x.round() as u32, home_data.y.round() as u32),
            size:           (home_data.width.round() as u32, home_data.height.round() as u32)
        }
    }
}