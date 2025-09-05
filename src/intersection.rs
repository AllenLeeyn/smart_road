use std::collections::HashMap;
use crate::car::Car;

use rand::prelude::IndexedRandom;
use rand::rng;
use sdl2::render::Texture;

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
}

impl<'a> Intersection<'a> {
    pub fn new() -> Self {
        use Direction::*;
        use Route::*;

        let mut cars_in = HashMap::new();

        for dir in [North, South, East, West] {
            for route in [Left, Straight, Right] {
                cars_in.insert((dir, route), Vec::new());
            }
        }
        Intersection { cars_in, cars_out: Vec::new() }
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
                let safe_distance = last_car.speed * 25;

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

        let car = Car::new(x, y, 33, 80, speed, texture, route, direction);

        println!("✅ Spawned car from {:?} with route {:?}", direction, route);
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
}

pub fn spawn_position(direction: Direction, route: Route) -> (i32, i32, i32) {
    match (direction, route) {
        // South-bound (cars move down → y = -80, they enter from top)
        (Direction::South, Route::Left)    => (408, -80, 5),
        (Direction::South, Route::Straight) => (358, -80, 5),
        (Direction::South, Route::Right)     => (308, -80, 7),

        // North-bound (cars move up → y = 900, they enter from bottom)
        (Direction::North, Route::Left)    => (458, 900, 5),
        (Direction::North, Route::Straight) => (508, 900, 5),
        (Direction::North, Route::Right)     => (558, 900, 7),

        // East-bound (cars move right → x = -80)
        (Direction::East, Route::Left)     => (-80, 438, 5),
        (Direction::East, Route::Straight)  => (-80, 488, 5),
        (Direction::East, Route::Right)      => (-80, 538, 7),

        // West-bound (cars move left → x = 900)
        (Direction::West, Route::Left)     => (900, 388, 5),
        (Direction::West, Route::Straight)  => (900, 338, 5),
        (Direction::West, Route::Right)      => (900, 288, 7),
    }
}