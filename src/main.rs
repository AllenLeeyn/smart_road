mod car;
mod cars_id;
mod consts;
mod intersection;
mod crossing_manager;
mod utils;

use intersection::{Intersection, Direction, Route};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use std::collections::HashMap;

use crate::consts::*;
use crate::utils::present_main_canvas;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _img_ctx = sdl2::image::init(InitFlag::PNG);

    let window = video_subsystem.window("smart-road", SIMULATION_WINDOW_WIDTH, SIMULATION_WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    // Load the background texture
    let texture_creator = canvas.texture_creator();
    let bg_texture = texture_creator.load_texture("assets/bg.png").unwrap();
    let car_right_texture = texture_creator.load_texture("assets/car_r.png").unwrap();
    let car_left_texture = texture_creator.load_texture("assets/car_l.png").unwrap();
    let car_straight_texture = texture_creator.load_texture("assets/car_s.png").unwrap();

    let car_textures_by_route: HashMap<Route, &Texture> = HashMap::from([
        (Route::Right, &car_right_texture),
        (Route::Left, &car_left_texture),
        (Route::Straight, &car_straight_texture),
    ]);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut intersection = Intersection::new(car_textures_by_route);

    use std::time::Instant;

    let target_frame_duration = BASE_DELTA_TIME;

    'running: loop {
        let frame_start = Instant::now();
        let events: Vec<_> = event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit { .. } => break 'running,

                Event::KeyDown { keycode: Some(key), .. } => match key {
                    Keycode::Escape => { 
                        show_statistics(&intersection, &sdl_context, &mut event_pump);
                        break 'running
                    },
                    Keycode::Down | Keycode::S => {
                        intersection.add_car_in(Direction::South);
                    }
                    Keycode::Up | Keycode::W => {
                        intersection.add_car_in(Direction::North);
                    }
                    Keycode::Left | Keycode::A => {
                        intersection.add_car_in(Direction::West);
                    }
                    Keycode::Right | Keycode::D => {
                        intersection.add_car_in(Direction::East);
                    }
                    Keycode::R => {
                        intersection.add_car_in_rnd();
                    }
                    _ => {} // Ignore other keys
                },

                _ => {}
            }
        }

        intersection.update();

        present_main_canvas(&mut canvas, &bg_texture, &intersection);

        let elapsed = frame_start.elapsed();
        if elapsed < target_frame_duration {
            std::thread::sleep(target_frame_duration - elapsed);
        }
    }
}

use sdl2::ttf::Font;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

fn show_statistics(
    intersection: &Intersection,
    sdl_context: &sdl2::Sdl,
    event_pump: &mut sdl2::EventPump
) {
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("Statistics", STATS_WINDOW_WIDTH, STATS_WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let font_path = "assets/Roboto-Regular.ttf"; // Change if needed
    let font: Font = ttf_context.load_font(font_path, FONT_SIZE).unwrap();

    let stats_text = intersection.get_statistics(); // should return String

    let surface = font
        .render(&stats_text)
        .blended_wrapped(Color::WHITE, 380)
        .unwrap();

    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let target = Rect::new(UI_PADDING_X, UI_PADDING_Y, surface.width(), surface.height());

    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
    canvas.copy(&texture, None, Some(target)).unwrap();
    canvas.present();

    'stat_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'stat_loop,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'stat_loop,
                _ => {}
            }
        }
    }
}
