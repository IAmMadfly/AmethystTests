
use bevy::prelude::{
    App, IntoQuerySystem, Commands
};

fn hello() {
    println!("Hello, world!");
    println!("Hello, world! AGAIN!!");
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
}

fn main() {
    App::build()
        .add_system(hello.system())
        .run();
}
