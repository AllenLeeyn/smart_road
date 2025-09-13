mod car;
mod cars_id;
mod intersection;
mod crossing_manager;
use intersection::{Intersection, Direction, Route};

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use std::time::Duration;
use std::collections::HashMap;
 
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

    let target_frame_duration = Duration::from_millis(1000 / 60); // ~16.67ms

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

        canvas.clear();
        canvas.copy(&bg_texture, None, None).unwrap();
        intersection.draw(&mut canvas);
        canvas.present();

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
        .window("Statistics", 400, 400)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let font_path = "assets/Roboto-Regular.ttf"; // Change if needed
    let font: Font = ttf_context.load_font(font_path, 20).unwrap();

    let stats_text = intersection.get_statistics(); // should return String

    let surface = font
        .render(&stats_text)
        .blended_wrapped(Color::WHITE, 380)
        .unwrap();

    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let target = Rect::new(10, 10, surface.width(), surface.height());

    canvas.set_draw_color(Color::RGB(30, 30, 30));
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
