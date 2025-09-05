mod car;
mod intersection;
use crate::intersection::{Intersection, Direction};

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let _img_ctx = sdl2::image::init(InitFlag::PNG);
 
    let window = video_subsystem.window("smart-road", 900, 900)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    
    // Load the background texture
    let texture_creator = canvas.texture_creator();
    let bg_texture = texture_creator.load_texture("bg.png").unwrap();
    let car_texture = texture_creator.load_texture("car.png").unwrap();


    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut intersection = Intersection::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,

                Event::KeyDown { keycode: Some(key), .. } => match key {
                    Keycode::Escape => break 'running,
                    Keycode::Down | Keycode::S => {
                        intersection.add_car_in(Direction::South, &car_texture);
                    }
                    Keycode::Up | Keycode::W => {
                        intersection.add_car_in(Direction::North, &car_texture);
                    }
                    Keycode::Left | Keycode::A => {
                        intersection.add_car_in(Direction::West, &car_texture);
                    }
                    Keycode::Right | Keycode::D => {
                        intersection.add_car_in(Direction::East, &car_texture);
                    }
                    _ => {} // Ignore other keys
                },

                _ => {}
            }
        }

        intersection.update();

        canvas.clear();
        canvas.copy(&bg_texture, None, None).unwrap();
        intersection.draw(&mut canvas);
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}