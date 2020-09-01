
use bevy::prelude::App;

fn hello() {
    println!("Hello, world!");
}

fn main() {
    App::build()
        .add_system(hello.system())
        .run();
}
