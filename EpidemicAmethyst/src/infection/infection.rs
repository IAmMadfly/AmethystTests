use tiled;

#[derive(Debug)]
enum HomeType {
    Appartment,
    House,
    Farm
}

#[derive(Debug)]
struct People {
    family_id:      u64,
    name:           String,
    infection:      Option<u64>
}

#[derive(Debug)]
struct Family {
    people:         Vec<People>
}

#[derive(Debug)]
pub struct Home {
    id:             u32,
    home_type:      HomeType,
    families:       Vec<Family>,
    location:       (u32, u32),
    size:           (u32, u32)
}

impl Home {
    fn new(home_data: tiled::Object) -> Self {
        Home {
            id:             home_data.id,
            home_type:      HomeType::Appartment,
            families:       self.generate_families(
                home_data.properties
            )
        }
    }
}