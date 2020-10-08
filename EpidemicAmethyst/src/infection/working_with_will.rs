use amethyst::{
    prelude::*,
    ecs::{Entity, Component, DenseVecStorage, DefaultVecStorage}
};


let family = world.create_entity()
    .with(Family::new().clone())    
    .build();

world.lazy_update().add(family, Family::new());

struct Family {
    pub fn new() -> Family {

    }
}

impl Component for Family {
    type Storage = DenseVecStorage<Self>;
}

struct Building {
    id:             u64,
    home_type:      HomeType
}

struct Occupants {
    people:         Vec<Entity>
}

impl Component for Occupants {
    type Storage = DenseVecStorage<Self>;
}

impl Building {
    pub fn new() -> Self {
        Building {
            id:             2,
            home_type:      HomeType::Apartment
        }
    }
}

enum Gender {
    Male,
    Female
}

impl Person {
    name:   String,
    gender: Gender,
    age:    u32,
}

struct Home {
    home:   Entity
}

fn main(world: &mut World) {
    let home        = Building::new();
    let occupants   = Occupants::new();

    let building = world.create_entity()
                        .with(location)
                        //.with(occupants)
                        .build();
    
    let person  = world.crate_entity()
                        .with(people_component_properties)
                        //.with(Home::new(building))
                        //.with(location)
                        .build();

    // Something here to bind relationship (adding components to known entities)
    // for both my people and buildings
    
    let people_pool = Vec<PeopleBuilder>;

}