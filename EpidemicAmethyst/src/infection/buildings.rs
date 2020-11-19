use amethyst::{
    prelude::*,
    ecs::{Component, DenseVecStorage, DefaultVecStorage, Entity},
};

pub struct Location {
    x:      f32,
    y:      f32
}

impl Component for Location {
    type Storage = DenseVecStorage<Self>;
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Location {
            x:  self.x,
            y:  self.y
        }
    }
}

impl Location {
    pub fn new (x: f32, y: f32) -> Self {
        Location {
            x,
            y
        }
    }

    pub fn block_x(&self) -> u32 {
        (self.x/32.0).floor() as u32
    }

    pub fn block_y(&self) -> u32 {
        (self.y/32.0).floor() as u32
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
    pub size:           [f32; 2],
    occupants:          Vec<(Entity, time::PrimitiveDateTime)>
}

impl Component for Building {
    type Storage    = DenseVecStorage<Self>;
}

impl Building {
    pub fn new (id: u32, size: [f32; 2]) -> Self {
        Building {
            _id:        id,
            size,
            occupants:  Vec::new()
        }
    }

    pub fn add_occupants(&mut self, new_occupants: Vec<Entity>, world: &World) {
        let curr_time = (*world
            .read_resource::<time::PrimitiveDateTime>())
            .clone();
        
        for new_occupant in new_occupants {
            self.occupants.push(
                (new_occupant, curr_time.clone())
            );
        }
    }
}

pub struct BuildingEntrance {
    pub location:   Location
}

impl Component for BuildingEntrance {
    type Storage = DenseVecStorage<Self>;
}

impl Clone for BuildingEntrance {
    fn clone(&self) -> Self {
        BuildingEntrance {
            location:   self.location.clone()
        }
    }
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
