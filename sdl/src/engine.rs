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

use sdl2::Sdl;

pub struct Game {
    pub running:        bool,
    sdl_context:        Option<Sdl>,
    window:             Option<sdl2::video::Window>
}

pub fn initialise() -> Game {
    let sdl_context = sdl2::init();
    
    match sdl_context {
        Ok(context) => {
            println!("Got sdl successfully");
            Game{
                running:        true, 
                sdl_context:    Some(context), 
                window:         None
            }
        }
        Err(e) => {
            println!("Error was: {}", e);
            Game{running: false, sdl_context: None, window: None}
        }
    }
}

pub fn get_window(mut game: Game, title: String) -> Game {
    if game.running {
        if let Some(s) = &game.sdl_context {
            println!("Returning game with Window!");
            game.window = Some(s.video().unwrap()
                                        .window(title.as_str(), 800, 600)
                                        .position_centered()
                                        .build()
                                        .unwrap());
        }
    }
    game
}

/*
pub fn update_components() {
    println!("Processing");
}

pub fn update_input() {
    println!("Taking and processing input");
}

pub fn destroy() {
    println!("Destroying game!");
}
*/