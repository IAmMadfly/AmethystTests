extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

fn check_option(opt:Option<u128>) {
    match opt {
        None => println!("It was none!"),
        Some(3) => println!("It was exactly 3!"),
        Some(_) => println!("It was some!")
    }
}

fn plus_one(i: &mut i32) {
    *i = *i + 1_i32;
}
 
fn main() {
    let mut poss: Option<u128> = None;
    check_option(poss);
    poss = Some(3);
    check_option(poss);

    let mut val: i32 = 0;
    plus_one(&mut val);
    println!("Value is: {}", val);

    println!("Hello, world!");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Karls freaking POWER!", 800, 600)
                                .resizable()
                                .borderless()
                                .position_centered()
                                .build()
                                .unwrap();

    //window.set_bordered(false);
    
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut dir = true;

    'running: loop {
        match dir {
            true => i =     i + 1,
            false => i =    i - 1
        }
        // Change colour direction
        if i == 0 || i == 255 {
            dir = !dir;
        }

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
