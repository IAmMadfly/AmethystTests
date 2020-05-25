
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

struct Pong;

pub enum Side {
    Left,
    Right
}

pub struct Paddle {
    pub side:   Side,
    pub width:  f32,
    pub height: f32
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width:  PADDLE_WIDTH
            height: PADDLE_HEIGHT
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
    }

    fn initialise_camera(world: &mut World) {

        let mut tramsform = Transform::default();

        transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

        world
            .create_entity()
            .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
            .with(transform)
            .build();
    }
}