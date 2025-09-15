use crate::car::Car;
use crate::cars_id::CarIdGenerator;
use crate::crossing_manager::CrossingManager;
use crate::utils::*;
use std::collections::HashMap;

use chrono::{DateTime, Local};
use sdl2::render::Texture;
use std::time::{SystemTime};

use crate::consts::*;

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
    pub car_textures: HashMap<Route, &'a Texture<'a>>,
    pub cars_in: HashMap<(Direction, Route), Vec<Car<'a>>>,
    pub cars_out: Vec<Car<'a>>,
    pub id_generator: CarIdGenerator,
    pub crossing_manager: CrossingManager,
    pub collision_count: usize,
    pub near_miss: usize,
    pub last_update: SystemTime,
}

impl<'a> Intersection<'a> {
    pub fn new(car_textures: HashMap<Route, &'a Texture<'a>>) -> Self {
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
            car_textures,
            cars_in, cars_out: Vec::new(),
            id_generator, crossing_manager, collision_count: 0, near_miss: 0,
            last_update: SystemTime::now() }
    }

    pub fn add_car_in_rnd(&mut self) {
        let direction = get_rnd_direction();
        self.add_car_in(direction);
    }

    pub fn add_car_in(&mut self, direction: Direction) {

        for route in get_rnd_routes() {
            let (x, y, speed) = get_spawn_position(direction, route);
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

                let texture = self.car_textures.get(&route).expect("Missing texture for route");
                
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

                self.cars_in.get_mut(&(direction, route)).unwrap().push(car);

                let datetime: DateTime<Local> = entry_time.into();
                println!(
                    "Spawned car {} heading {:?} going {:?} | Entry time: {}",
                    car_id,
                    direction,
                    route,
                    datetime.format("%H:%M:%S%.3f")
                );
                return; // Successfully spawned, exit function
            }
        }

        // If reached here, no lane available
        println!(
            "No free lane found for spawning car in direction {:?}",
            direction
        );
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
                    println!("Collision between {} and {}", a.id, b.id);
                    a.collided = true;
                    b.collided = true;
                    self.collision_count += 1;
                }
            }
        }
    }

    pub fn update(&mut self) {
        let now = SystemTime::now();
        let delta = now.duration_since(self.last_update).unwrap_or(BASE_DELTA_TIME);
        self.last_update = now;

        self.check_cars_collision();
        self.crossing_manager.update();
        for queue in self.cars_in.values_mut() {
            let mut i = 0;

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

                if !car.brake && should_brake { self.near_miss += 1}
                car.brake = should_brake;
                car.update(delta);

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
            return "No vehicles have crossed the intersection yet.".to_string();
        }

        let (min_speed, max_speed, avg_speed) = calculate_speed_statistics(cars);
        let (min_duration, max_duration, avg_duration) = calculate_duration_statistics(cars);

        format!(
            "Intersection Statistics\n\
            -----------------------------\n\
            Vehicles Crossed: {}\n\
            Collisions: {}\n\
            Near Misses: {}\n\
            \n\
            Max Speed: {} px/s\n\
            Min Speed: {} px/s\n\
            Avg Speed: {} px/s\n\
            \n\
            Max Time in Intersection: {} s\n\
            Min Time in Intersection: {} s\n\
            Avg Time in Intersection: {} s",
            total,
            self.collision_count,
            self.near_miss,
            max_speed,
            min_speed,
            avg_speed,
            max_duration,
            min_duration,
            avg_duration
        )
    }
}
