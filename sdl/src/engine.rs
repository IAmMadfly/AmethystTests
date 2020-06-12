// Karls game engine
// Process:
/*
-> Process input 
-> Update components
-> Render new information

*/
extern crate sdl2;
extern crate sdl2_sys;

//use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

//use std::time::Duration;

struct Size {
    width:      u32,
    height:     u32
}

pub struct Game {
    running:            bool,
    sdl_context:        sdl2::Sdl,
    canvas:             Option<sdl2::render::Canvas<sdl2::video::Window>>,
    elements:           Vec<Element>
}

struct Element {
    point:              Point,
    z:                  u32,
    size:               Size
}

struct Point {
    x:                  i32,
    y:                  i32
}

impl Game {
    pub fn new() -> Game {
        Game{
            running:            false,
            sdl_context:        sdl2::init().expect("Failed to get SDL context!"),
            canvas:             None,
            elements:           Vec::new()
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn initialize(&mut self, width: u32, height: u32) {
        self.canvas = get_window_contents(&self.sdl_context,
            Size{
                width,
                height
            }
        );
        self.running = true;
        self.elements.push(
            Element{
                point:      Point{x:0, y:0},
                z:          1,
                size:       Size{height:5, width:5}
            }
        );
    }

    pub fn render(&self) {
        
    }

    pub fn process_input(&mut self) {
        for event in self.sdl_context.event_pump().expect("Failed to get event pump!").poll_iter() {
            match event {
                Event::Quit {..} | 
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => self.running = false,
                Event::KeyDown {keycode: Some(key), ..} => process_key(key, &mut self.elements),
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        
    }
}

fn process_key(key: Keycode, elements: &mut Vec<Element>) {
    match key {
        Keycode::W => println!("Up"),
        Keycode::S => println!("Down"),
        Keycode::A => println!("Left"),
        Keycode::D => println!("Right"),
        _ => println!("Keycode: {}", key)
    }
}

fn get_window_contents(sdl: &sdl2::Sdl, size: Size) -> Option<sdl2::render::Canvas<sdl2::video::Window>> {
    match sdl.video() {
        Ok(vid) => {
            match vid.window("Epidemic", size.height, size.width).position_centered().build() {
                Ok(wind) => Some(wind.into_canvas().build().expect("Failed to get canvas!!")),
                Err(e) => {
                    println!("Failed to get window, Err: {}", e);
                    panic!("No windowContents, no game!");
                }
            }
        },
        Err(e) => {
            println!("Failed to get Video subsystem. Err: {}", e);
            panic!("No windowContents, no game!");
        }
    }
}
