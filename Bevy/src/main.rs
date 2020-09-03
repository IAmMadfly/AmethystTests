
use bevy::{prelude::{
    App, IntoQuerySystem, 
    Commands, IntoForEachSystem, AddDefaultPlugins
    }
};

//use bevy::prelude::*;

fn hello() {
    println!("Hello, world!");
    println!("Hello, world! AGAIN!!");
}

struct Person;
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Something".to_owned())));
}

fn greet_people(_person: &Person, name: &Name) {
    println!("Hello {}!", name.0);
}

fn main() {
    App::build()
        //.add_plugin(CorePlugin::default())
        //.add_plugin(InputPlugin::default())
        //.add_plugin(WindowPlugin::default())
        .add_default_plugins()
        .add_startup_system(add_people.system())
        .add_system(greet_people.system())
        .add_system(hello.system())
        .run();
}
