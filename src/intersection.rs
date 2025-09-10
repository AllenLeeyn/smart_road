use std::collections::HashMap;
use crate::car::Car;
use crate::cars_id::CarIdGenerator;
use crate::crossing_manager::CrossingManager;

use rand::prelude::IndexedRandom;
use rand::rng;
use sdl2::render::Texture;
use std::time::Duration;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Route {
    Left,
    Right,
    Straight,
}

pub struct Intersection<'a> {
    pub cars_in: HashMap<(Direction, Route), Vec<Car<'a>>>,
    pub cars_out: Vec<Car<'a>>,
    pub id_generator: CarIdGenerator,
    pub crossing_manager: CrossingManager,
}

impl<'a> Intersection<'a> {
    pub fn new() -> Self {
        use Direction::*;
        use Route::*;

        let mut cars_in = HashMap::new();
        let id_generator = CarIdGenerator::new();
        let crossing_manager = CrossingManager::new();

        for dir in [North, South, East, West] {
            for route in [Left, Straight, Right] {
                cars_in.insert((dir, route), Vec::new());
            }
        }
        Intersection { cars_in, cars_out: Vec::new(), id_generator, crossing_manager }
    }

    pub fn add_car_in_rnd(&mut self, texture: &'a Texture<'a>) {
        let direction = get_rnd_direction();
        self.add_car_in(direction, &texture);
    }

    pub fn add_car_in(&mut self, direction: Direction, texture: &'a Texture<'a>) {
        let route = get_rnd_route();
        let (x, y, speed) = spawn_position(direction, route);
        let lane = self.cars_in.get(&(direction, route)).unwrap();

        // Check safe distance from the last car in lane (if any)
        let can_spawn = match lane.last() {
            Some(last_car) => {
                let safe_distance = 80 + 50 + last_car.speed * 3;

                match direction {
                    Direction::South => last_car.y - y >= safe_distance,
                    Direction::North => y - last_car.y >= safe_distance,
                    Direction::East  => last_car.x - x >= safe_distance,
                    Direction::West  => x - last_car.x >= safe_distance,
                }
            }
            None => true, // Lane is empty, okay to spawn
        };

        if !can_spawn {
            println!("🚫 Too close to last car in {:?} {:?}", direction, route);
            return;
        }

        let car_id = self.id_generator.get_next(direction, route);
        let distance_to_entry = if route == Route::Right { 300.0 } else { 350.0 };
        let entry_time = self.crossing_manager.latest_available_time(
                                            direction, route, distance_to_entry);
        self.crossing_manager.reserve_path(&car_id, direction, route, distance_to_entry);

        let car = Car::new(
            car_id.clone(), x, y, 33, 80, 
            speed, texture, route, entry_time, direction);

       let datetime: DateTime<Local> = entry_time.into();
        println!(
            "✅ Spawned car {} heading {:?} going {:?} | Entry time: {}",
            car_id, direction, route, datetime.format("%H:%M:%S%.3f")
        );
        self.cars_in.get_mut(&(direction, route)).unwrap().push(car);
    }

    pub fn update(&mut self) {
        for queue in self.cars_in.values_mut() {
            let mut i = 0;

            while i < queue.len() {
                let car = &mut queue[i];
                car.update();

                if car.exited {
                    let exited_car = queue.remove(i);
                    self.cars_out.push(exited_car);
                } else {
                    i += 1;
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        for queue in self.cars_in.values() {
            for car in queue {
                car.draw(canvas);
            }
        }
    }

    pub fn get_statistics(&self) -> String {
        let cars = &self.cars_out;

        let total = cars.len();

        if total == 0 {
            return "📊 No vehicles have crossed the intersection yet.".to_string();
        }

        // Initialize with first car's values
        let mut min_speed = f32::MAX;
        let mut max_speed = f32::MIN;

        let mut min_duration = Duration::MAX;
        let mut max_duration = Duration::ZERO;

        for car in cars {
            if let Some(exit_time) = car.time_exit {
                if let Ok(duration) = exit_time.duration_since(car.time_enter) {
                    // Convert duration to seconds as f32
                    let duration_secs = duration.as_secs_f32();

                    if duration_secs > 0.0 {
                        let distance = car.dist as f32;
                        let effective_speed = distance / duration_secs;

                        // Update speed stats
                        min_speed = min_speed.min(effective_speed);
                        max_speed = max_speed.max(effective_speed);
                    }

                    // Update duration stats
                    min_duration = min_duration.min(duration);
                    max_duration = max_duration.max(duration);
                }
            }
        }

        format!(
            "Intersection Statistics\n\
             -----------------------------\n\
             Vehicles Crossed: {}\n\
             Max Speed: {} px/s\n\
             Min Speed: {} px/s\n\
             Max Time in Intersection: {} s\n\
             Min Time in Intersection: {} s",
            total,
            round_two(max_speed),
            round_two(min_speed),
            round_two(min_duration.as_secs_f32()),
            round_two(max_duration.as_secs_f32())
        )
    }
}

pub fn spawn_position(direction: Direction, route: Route) -> (i32, i32, i32) {
    match (direction, route) {
        (Direction::South, Route::Left) => (408, -80, 5),
        (Direction::South, Route::Straight) => (358, -80, 5),
        (Direction::South, Route::Right) => (308, -80, 7),

        (Direction::North, Route::Left) => (458, 900, 5),
        (Direction::North, Route::Straight) => (508, 900, 5),
        (Direction::North, Route::Right) => (558, 900, 7),

        (Direction::East, Route::Left) => (-80, 438, 5),
        (Direction::East, Route::Straight) => (-80, 488, 5),
        (Direction::East, Route::Right) => (-80, 538, 7),

        (Direction::West, Route::Left) => (900, 388, 5),
        (Direction::West, Route::Straight) => (900, 338, 5),
        (Direction::West, Route::Right) => (900, 288, 7),
    }
}

fn round_two(n: f32) -> f32 {
    (n * 100.0).round() / 100.0
}

pub fn get_rnd_route() -> Route {
    let routes = [Route::Left, Route::Right, Route::Straight];
    let mut rng = rng();
    *routes.choose(&mut rng).unwrap()
}

pub fn get_rnd_direction() -> Direction {
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    let mut rng = rng();
    *directions.choose(&mut rng).unwrap()
}
