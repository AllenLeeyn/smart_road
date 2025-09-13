mod car;
mod cars_id;
mod intersection;
mod crossing_manager;
use intersection::{Intersection, Direction};

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::env;

// For testing
use car::Car;
use std::time::SystemTime;

pub fn main() {
    // Using args because I was having issues with normal/official testing
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "test" => {
                run_test_mode();
                return;
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                println!("Either run the program without arguments or use 'cargo run test'");
                return;
            }
        }
    }
    
    // If no args, run normally
    run_main_simulation();
}

fn run_main_simulation() {
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
    let car_texture = texture_creator.load_texture("assets/car.png").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut intersection = Intersection::new();
    'running: loop {
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
                    Keycode::R => {
                        intersection.add_car_in_rnd(&car_texture);
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

fn run_test_mode() {
    println!("=== TEST MODE ===");

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
    let car_texture = texture_creator.load_texture("assets/car.png").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut intersection = Intersection::new();

    let start_time = SystemTime::now();
    let end_time = SystemTime::now() + Duration::from_secs(10);

    let mut success = false;

    loop {
        intersection.add_car_in_rnd(&car_texture);
        intersection.update();
        canvas.clear();
        canvas.copy(&bg_texture, None, None).unwrap();
        intersection.draw(&mut canvas);
        canvas.present();
        if intersection.cars_out.len() > 10000 {
            println!("10000 cars have crossed the intersection, no issues found");
            success = true;
            break;
        } else if intersection.collision_count > 0 {
            println!("Collision detected");
            let collided_cars = intersection.cars_in.values().flatten().filter(|car| car.collided).collect::<Vec<&Car>>();
            println!("Collided cars: {:?}", collided_cars.len());
            for car in collided_cars {
                println!("Collision at: {} {:?}", car.id, car.direction);
            }
            success = false;
            break;
        } else if SystemTime::now() > end_time {
            println!("10 seconds have passed");
            success = false;
            break;
        }
    }
    if success {
        println!("Test passed");
        show_statistics(&intersection, &sdl_context, &mut event_pump);
    } else {
        println!("Test failed");
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
    event_pump: &mut sdl2::EventPump,) {
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