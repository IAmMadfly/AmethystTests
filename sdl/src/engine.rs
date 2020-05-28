// Karls game engine
// Process:
/*
-> Process input 
-> Update components
-> Render new information

*/
extern crate sdl2;

//use sdl2::pixels::Color;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

//use std::time::Duration;

struct Game {
    running: bool
}


pub fn update_components() {
    println!("Processing");
}

pub fn update_input() {
    println!("Taking and processing input");
}

pub fn destroy() {
    println!("Destroying game!");
}