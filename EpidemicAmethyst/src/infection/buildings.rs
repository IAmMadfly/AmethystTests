use amethyst::{
    prelude::*,
    ecs::{Entity, Component, DenseVecStorage, DefaultVecStorage, EntityBuilder}
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
    pub fn new (x: f32, y: f32) -> Self {
        Location {
            x,
            y
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

pub struct Building {
    _id:                u32,
    pub size:           [f32; 2]
}

impl Component for Building {
    type Storage    = DenseVecStorage<Self>;
}

impl Building {
    pub fn new (id: u32, size: [f32; 2]) -> Self {
        Building {
            _id: id,
            size
        }
    }
}

pub struct BuildingEntrance {
    location:   Location
}

impl Component for BuildingEntrance {
    type Storage = DenseVecStorage<Self>;
}

impl BuildingEntrance {
    pub fn new(location: Location) -> Self {
        BuildingEntrance {
            location
        }
    }
}

#[derive(Default)]
pub struct MaxOccupants {
    max_occupants:      usize
}

impl Component for MaxOccupants {
    type Storage    = DefaultVecStorage<Self>;
}

impl MaxOccupants {
    pub fn new(max_occupants: usize) -> Self {
        MaxOccupants {
            max_occupants
        }
    }

    pub fn get_max_occupants(&self) -> usize {
        self.max_occupants
    }
}
