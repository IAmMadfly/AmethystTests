// Karls game engine
// Process:
/*
-> Process input 
-> Update components
-> Render new information

*/
extern crate sdl2;

//use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

//use std::time::Duration;

pub struct Game {
    running:            bool,
    sdl_context:        sdl2::Sdl,
    window:             Option<sdl2::video::Window>,
    event_pump:         Option<sdl2::EventPump>,
    elements:           Elements
}

struct Elements {
    elements:           Vec<Element>
}

struct Element {
    word:               String,
    point:              Point,
    size:               Size
}

struct Point {
    x:                  i32,
    y:                  i32
}

struct Size {
    x:                  i32,
    y:                  i32
}

impl Elements {
    fn default() -> Elements {
        Elements{
            elements: Vec::<Element>::new()
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game{
            running:            true,
            sdl_context:        sdl2::init().unwrap(),
            window:             None,
            event_pump:         None,
            elements:           Elements::default()
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn initialize(&mut self) {
        self.window = get_window(&self.sdl_context);
    }

    pub fn process_input(&mut self) {
        if self.event_pump.is_some() {
            let event_poll = self.event_pump.as_mut().unwrap().poll_iter();
                for event in event_poll {
                    match event {
                        Event::Quit {..} | 
                        Event::KeyDown {keycode: Some(Keycode::Escape), .. } => self.running = false,
                        Event::KeyDown {keycode: Some(key), ..} => process_key(key, &mut self.elements),
                        _ => println!("Different event!")
                    }
                }
        } else {
            println!("No event pump available!");
        }
    }

    pub fn update(&mut self) {

    }
}

fn process_key(key: Keycode, elements: &mut Elements) {
    match key {
        Keycode::W => println!("Up"),
        Keycode::S => println!("Down"),
        Keycode::A => println!("Left"),
        Keycode::D => println!("Right"),
        _ => println!("Keycode: {}", key)
    }
}

fn get_window(sdl: &sdl2::Sdl) -> Option<sdl2::video::Window> {
    match sdl.video() {
        Ok(vid) => {
            match vid.window("Game", 800, 600).position_centered().build() {
                Ok(wind) => Some(wind),
                Err(e) => {
                    println!("Failed to get window, Err: {}", e);
                    None
                }
            }
        },
        Err(e) => {
            println!("Failed to get Video subsystem. Err: {}", e);
            None
        }
    }
}
