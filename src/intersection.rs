use crate::car::Car;
use crate::cars_id::CarIdGenerator;
use crate::crossing_manager::CrossingManager;
use crate::consts::*;
use std::collections::HashMap;

use chrono::{DateTime, Local};
use rand::prelude::IndexedRandom;
use rand::rng;
use sdl2::render::Texture;
use std::time::Duration;

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
    pub collision_count: usize,
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
        Intersection {
            cars_in,
            cars_out: Vec::new(),
            id_generator,
            crossing_manager,
            collision_count: 0,
        }
    }

    pub fn add_car_in_rnd(&mut self, texture: &'a Texture<'a>, no_print: bool) {
        let direction = get_rnd_direction();
        self.add_car_in(direction, &texture, no_print);
    }

    pub fn add_car_in(&mut self, direction: Direction, texture: &'a Texture<'a>, no_print: bool) {
        for route in get_rnd_routes() {
            let (x, y, speed) = spawn_position(direction, route);
            let lane = self.cars_in.get(&(direction, route)).unwrap();
            let can_spawn = car_spawn_check(lane, direction, x, y, CAR_HEIGHT_PX as i32);

            if can_spawn {
                let car_id = self.id_generator.get_next(direction, route);
                let distance_to_entry = if route == Route::Right { ENTRY_DISTANCE_PX_RIGHT as f64 } else { ENTRY_DISTANCE_PX as f64 };
                let entry_time = self.crossing_manager.latest_available_time(
                    direction,
                    route,
                    distance_to_entry,
                );
                self.crossing_manager
                    .reserve_path(&car_id, direction, route, distance_to_entry);

                let car = Car::new(
                    car_id.clone(),
                    x,
                    y,
                    CAR_WIDTH_PX,
                    CAR_HEIGHT_PX,
                    speed,
                    texture,
                    route,
                    entry_time,
                    direction,
                );

                let datetime: DateTime<Local> = entry_time.into();
                if !no_print {
                    println!(
                    "✅ Spawned car {} heading {:?} going {:?} | Entry time: {}",
                    car_id,
                    direction,
                    route,
                    datetime.format("%H:%M:%S%.3f")
                    );
                }
                self.cars_in.get_mut(&(direction, route)).unwrap().push(car);
                return; // Successfully spawned, exit function
            }
        }

        // If reached here, no lane available
        if !no_print {
            println!(
            "🚫 No free lane found for spawning car in direction {:?}",
            direction
            );
        }
    }

    fn check_cars_collision(&mut self) {
        let mut cars: Vec<&mut Car> = Vec::new();

        for queue in self.cars_in.values_mut() {
            for car in queue.iter_mut() {
                cars.push(car);
            }
        }

        let len = cars.len();
        for i in 0..len {
            for j in (i + 1)..len {
                // SAFELY get two mutable references without aliasing using split_at_mut
                let (left, right) = cars.split_at_mut(j);
                let a = &mut left[i];
                let b = &mut right[0];

                if (!a.collided || !b.collided) && a.intersects(b) {
                    println!("💥 Collision between {} and {}", a.id, b.id);
                    a.collided = true;
                    b.collided = true;
                    self.collision_count += 1;
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.check_cars_collision();
        self.crossing_manager.update();
        for queue in self.cars_in.values_mut() {
            let mut i = 0;

            // Collect car IDs before the loop to avoid borrow checker issues
            // let car_ids: Vec<_> = queue.iter().map(|c| &c.id).collect();
            // println!("Cars in queue: {:?}", car_ids);

            while i < queue.len() {
                // Check if there's a front car and if current car should brake
                let should_brake = if i > 0 {
                    // Get immutable reference to front car first
                    let front_car = &queue[i - 1];
                    let current_car = &queue[i];
                    !current_car.collided && current_car.is_too_close(front_car)
                } else {
                    false
                };

                // Now get mutable reference to current car
                let car = &mut queue[i];

                if car.collided {
                    i += 1;
                    continue;
                }

                car.brake = should_brake;
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
        let _ = self.crossing_manager.draw(canvas);
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
             Collisions: {}\n\
             Max Speed: {} px/s\n\
             Min Speed: {} px/s\n\
             Max Time in Intersection: {} s\n\
             Min Time in Intersection: {} s",
            total,
            self.collision_count,
            round_two(max_speed),
            round_two(min_speed),
            round_two(max_duration.as_secs_f32()),
            round_two(min_duration.as_secs_f32())
        )
    }
}

pub fn spawn_position(direction: Direction, route: Route) -> (i32, i32, i32) {
    match (direction, route) {
        (Direction::South, Route::Left) => SPAWN_POSITION_SOUTH_LEFT,
        (Direction::South, Route::Straight) => SPAWN_POSITION_SOUTH_STRAIGHT,
        (Direction::South, Route::Right) => SPAWN_POSITION_SOUTH_RIGHT,

        (Direction::North, Route::Left) => SPAWN_POSITION_NORTH_LEFT,
        (Direction::North, Route::Straight) => SPAWN_POSITION_NORTH_STRAIGHT,
        (Direction::North, Route::Right) => SPAWN_POSITION_NORTH_RIGHT,

        (Direction::East, Route::Left) => SPAWN_POSITION_EAST_LEFT,
        (Direction::East, Route::Straight) => SPAWN_POSITION_EAST_STRAIGHT,
        (Direction::East, Route::Right) => SPAWN_POSITION_EAST_RIGHT,

        (Direction::West, Route::Left) => SPAWN_POSITION_WEST_LEFT,
        (Direction::West, Route::Straight) => SPAWN_POSITION_WEST_STRAIGHT,
        (Direction::West, Route::Right) => SPAWN_POSITION_WEST_RIGHT,
    }
}

fn car_spawn_check(lane: &Vec<Car>, direction: Direction, x: i32, y: i32, height: i32) -> bool {
    match lane.last() {
        Some(last_car) => {
            let safe_distance = 50.max(last_car.speed * 15);
            let last_bb = last_car.bounding_box();
            match direction {
                Direction::North => y >= last_bb.y() + last_bb.height() as i32 + safe_distance,
                Direction::South => y + height + safe_distance <= last_bb.y(),
                Direction::East => x + height + safe_distance <= last_bb.x(),
                Direction::West => x >= last_bb.x() + last_bb.width() as i32 + safe_distance,
            }
        }
        None => true,
    }
}

fn round_two(n: f32) -> f32 {
    (n * 100.0).round() / 100.0
}

use rand::prelude::SliceRandom;
pub fn get_rnd_routes() -> Vec<Route> {
    let mut routes = vec![Route::Left, Route::Right, Route::Straight];
    let mut rng = rng();
    routes.shuffle(&mut rng);
    routes
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
