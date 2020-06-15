// Karls game engine
// Process:
/*
-> Process input 
-> Update components
-> Render new information

*/
extern crate sdl2;
extern crate sdl2_sys;

mod element;
mod tools;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

//use std::time::Duration;

pub struct Game {
    running:            bool,
    sdl_context:        sdl2::Sdl,
    canvas:             sdl2::render::Canvas<sdl2::video::Window>,
    elements:           Vec<Box<dyn element::Element>>,
    last_time_update:   std::time::Instant,
    last_time_render:   std::time::Instant
}

impl Game {
    pub fn new() -> Game {
        let sdl = sdl2::init().expect("Failed to get SDL context!");
        Game{
            running:            false,
            canvas:             get_window_contents(
                                    &sdl, tools::Size{width: 800, height: 600}
                                ),
            sdl_context:        sdl,
            elements:           Vec::new(),
            last_time_update:   std::time::Instant::now(),
            last_time_render:   std::time::Instant::now()
        }
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn initialize(&mut self, width: u32, height: u32) {
        self.running = true;
        self.elements.push(
            Box::new(
                element::Object{
                    point:      element::Point{x:0, y:0},
                    z:          1,
                    size:       element::tools::Size{height:5, width:5}
                }
            )
        );
    }

    pub fn render(&mut self) {
        if (self.last_time_render.elapsed().as_millis() as f32) < 16.5 {
            return
        }
        self.last_time_render = std::time::Instant::now();

        if let Some(ele) = self.elements.get(0) {
            let point = ele.getPos();

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            self.canvas.set_draw_color(Color::RED);
            self.canvas.draw_rect(
                Rect::new(
                    point.x,
                    point.y,
                    5,
                    5
                )
            );
            self.canvas.present();
        }
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
        if self.last_time_update.elapsed().as_millis() > 50 {
            return
        }
        self.last_time_update = std::time::Instant::now();

        let element = self.elements.get_mut(0);

        if let Some(ele) = element {
            ele.movePos(1,1);
        }
    }
}

fn process_key(key: Keycode, elements: &mut Vec<Box<dyn element::Element>>) {
    match key {
        Keycode::W => println!("Up"),
        Keycode::S => println!("Down"),
        Keycode::A => println!("Left"),
        Keycode::D => println!("Right"),
        _ => println!("Keycode: {}", key)
    }
}

fn get_window_contents(sdl: &sdl2::Sdl, size: tools::Size) -> sdl2::render::Canvas<sdl2::video::Window> {
    match sdl.video() {
        Ok(vid) => {
            match vid.window("Epidemic", size.width, size.height).position_centered().build() {
                Ok(wind) => wind.into_canvas().build().expect("Failed to get canvas!!"),
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
