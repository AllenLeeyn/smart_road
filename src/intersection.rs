use crate::car::Car;
use crate::cars_id::CarIdGenerator;
use std::collections::HashMap;

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
}

impl<'a> Intersection<'a> {
    pub fn new() -> Self {
        use Direction::*;
        use Route::*;

        let mut cars_in = HashMap::new();
        let id_generator = CarIdGenerator::new();

        for dir in [North, South, East, West] {
            for route in [Left, Straight, Right] {
                cars_in.insert((dir, route), Vec::new());
            }
        }
        Intersection {
            cars_in,
            cars_out: Vec::new(),
            id_generator,
        }
    }

    pub fn add_car_in_rnd(&mut self, texture: &'a Texture<'a>) {
        let mut rng = rng();
        let directions = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        let direction = *directions.choose(&mut rng).unwrap();
        self.add_car_in(direction, &texture);
    }

    pub fn add_car_in(&mut self, direction: Direction, texture: &'a Texture<'a>) {
        let mut rng = rng();
        let routes = [Route::Left, Route::Right, Route::Straight];
        let route = *routes.choose(&mut rng).unwrap();

        // Adjust position and speed based on direction
        let (x, y, speed) = spawn_position(direction, route);

        // Access lane
        let lane = self.cars_in.get(&(direction, route)).unwrap();

        // Check safe distance from the last car in lane (if any)
        let can_spawn = match lane.last() {
            Some(last_car) => {
                let safe_distance = 50 + last_car.speed * 15;

                match direction {
                    Direction::South => last_car.y - y >= safe_distance,
                    Direction::North => y - last_car.y >= safe_distance,
                    Direction::East => last_car.x - x >= safe_distance,
                    Direction::West => x - last_car.x >= safe_distance,
                }
            }
            None => true, // Lane is empty, okay to spawn
        };

        if !can_spawn {
            println!("🚫 Too close to last car in {:?} {:?}", direction, route);
            return;
        }

        let car = Car::new(
            self.id_generator.get_next(direction, route),
            x,
            y,
            33,
            80,
            speed,
            texture,
            route,
            direction,
        );

        println!("✅ Spawned car heading {:?} going {:?}", direction, route);
        self.cars_in.get_mut(&(direction, route)).unwrap().push(car);
    }

    pub fn update(&mut self) {
        // Collect all cars in intersection and approaching for collision checking
        let mut intersection_cars = Vec::new();
        let mut all_cars_positions = Vec::new();

        for queue in self.cars_in.values() {
            for car in queue {
                // Cars in intersection
                if car.x >= 250 && car.x <= 650 && car.y >= 250 && car.y <= 650 {
                    intersection_cars.push((car.id.clone(), car.direction, car.route));
                }
                // All cars for distance checking
                all_cars_positions.push((car.id.clone(), car.x, car.y, car.direction, car.route));
            }
        }

        // Simple one-at-a-time intersection policy to prevent deadlocks
        let intersection_occupied = !intersection_cars.is_empty();

        // update cars
        for queue in self.cars_in.values_mut() {
            // Calculate which cars should stop due to rear-end prevention
            let mut should_stop_for_following = vec![false; queue.len()];
            for i in 1..queue.len() {
                let front_car = &queue[i - 1];
                let following_car = &queue[i];

                // Calculate distance between cars
                let dx = (front_car.x - following_car.x).abs();
                let dy = (front_car.y - following_car.y).abs();
                let distance = ((dx as f64 * dx as f64 + dy as f64 * dy as f64).sqrt()) as i32;
                let safe_distance = 50 + following_car.speed * 10;

                if distance < safe_distance {
                    should_stop_for_following[i] = true;
                }
            }

            let mut i = 0;
            while i < queue.len() {
                let car = &mut queue[i];

                // Check if car should yield based on intersection state
                let should_yield = Self::car_should_yield(car, &intersection_cars);

                let approaching = match car.direction {
                    Direction::North => car.y > 650 && car.y < 750,
                    Direction::South => car.y < 250 && car.y > 150,
                    Direction::East => car.x < 250 && car.x > 150,
                    Direction::West => car.x > 650 && car.x < 750,
                };

                let in_intersection = car.x >= 250 && car.x <= 650 && car.y >= 250 && car.y <= 650;

                // Deadlock-free policy: Only one car allowed in intersection at a time
                if approaching && intersection_occupied && !in_intersection {
                    // Wait if intersection is occupied and car is approaching
                    car.speed = 0;
                } else if should_stop_for_following[i] {
                    // Stop if too close to car ahead in same lane
                    car.speed = 0;
                } else if car.speed == 0
                    && (!intersection_occupied || in_intersection)
                    && !should_stop_for_following[i]
                {
                    // Restore speed when intersection is free OR car is already inside
                    car.speed = match car.route {
                        Route::Right => 7,
                        _ => 5,
                    };
                }

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

    fn car_should_yield(car: &Car, intersection_cars: &[(String, Direction, Route)]) -> bool {
        // Check if car is approaching intersection
        let car_approaching = match car.direction {
            Direction::North => car.y > 650,
            Direction::South => car.y < 250,
            Direction::East => car.x < 250,
            Direction::West => car.x > 650,
        };

        if !car_approaching {
            return false; // Car already in intersection doesn't need to yield
        }

        // Real traffic rules:
        // 1. Approaching cars yield to cars already in intersection
        // 2. Left turns yield to straight and right from opposite direction

        for (other_id, other_direction, other_route) in intersection_cars {
            // Skip self
            if *other_id == car.id {
                continue;
            }

            // Always yield to cars already in intersection
            return true;
        }

        false
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
