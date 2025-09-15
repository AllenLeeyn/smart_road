use crate::intersection::{Intersection, Direction, Route};
use crate::crossing_manager::{ZoneIndex, ZoneReservation};
use crate::consts::*;
use crate::car::Car;
use std::time::{SystemTime, Duration};
use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::prelude::*;
use rand::rng;

pub fn present_main_canvas(canvas: &mut Canvas<Window>, bg_texture: &Texture, intersection: &Intersection) {
    canvas.clear();
    canvas.copy(&bg_texture, None, None).unwrap();
    intersection.draw(canvas);
    canvas.present();
}

pub fn round_two(n: f32) -> f32 {
    (n * 100.0).round() / 100.0
}

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

pub fn route_to_zone_path(dir: Direction, route: Route) -> Vec<ZoneIndex> {
    match (dir, route) {
        (Direction::South, Route::Left) => ZONES_FOR_SOUTH_LEFT.to_vec(),
        (Direction::South, Route::Straight) => ZONES_FOR_SOUTH_STRAIGHT.to_vec(),

        (Direction::North, Route::Left) => ZONES_FOR_NORTH_LEFT.to_vec(),
        (Direction::North, Route::Straight) => ZONES_FOR_NORTH_STRAIGHT.to_vec(),

        (Direction::East, Route::Left)  => ZONES_FOR_EAST_LEFT.to_vec(),
        (Direction::East, Route::Straight) => ZONES_FOR_EAST_STRAIGHT.to_vec(),

        (Direction::West, Route::Left) => ZONES_FOR_WEST_LEFT.to_vec(),
        (Direction::West, Route::Straight) => ZONES_FOR_WEST_STRAIGHT.to_vec(),

        (_, Route::Right) => vec![],
    }
}

pub fn generate_zone_reservations(
    car_id: &str,
    path: &[ZoneIndex],
    entry_time: SystemTime,
) -> Vec<(ZoneIndex, ZoneReservation)> {
    let zone_time = Duration::from_secs_f64(ZONE_LENGTH_PX / SPEED_PX_PER_SEC);
    let occupy_time = Duration::from_secs_f64(CAR_HEIGHT_PX as f64 / SPEED_PX_PER_SEC);
    let safe_gap = Duration::from_secs_f64(SAFE_DISTANCE_PX / SPEED_PX_PER_SEC);

    let mut reservations = Vec::new();

    for (i, &zone) in path.iter().enumerate() {
        let time_in = entry_time + zone_time * i as u32;
        let time_out = time_in + zone_time + occupy_time + safe_gap;

        let reservation = ZoneReservation {
            _car_id: car_id.to_string(),
            time_in,
            time_out,
        };

        reservations.push((zone, reservation));
    }

    reservations
}

pub fn get_spawn_position(direction: Direction, route: Route) -> (i32, i32, i32) {
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

pub fn car_spawn_check(lane: &Vec<Car>, direction: Direction, x: i32, y: i32, height: i32) -> bool {
    if lane.len() >= 4 { return false; }

    match lane.last() {
        Some(last_car) => {
            let safe_distance = height;
            let last_bb = last_car.bounding_box();
            is_safe_distance_from_car(direction, x, y, height, &last_bb, safe_distance)
        }
        None => true,
    }
}

pub fn get_angle_by_direction(direction: Direction) -> f64 {
    match direction {
        Direction::South => 0.0,
        Direction::North => 180.0,
        Direction::East => 270.0,
        Direction::West => 90.0,
    }
}

pub fn get_direction_offset_by_direction(direction: Direction, width: u32, height: u32) -> (i32, i32) {
    match direction {
        Direction::North | Direction::South => (0, 0),
        Direction::East | Direction::West => {
            ((height as i32 - width as i32) / 2, -(height as i32 - width as i32) / 2)
        }
    }
}

pub fn get_distance_by_direction(x: i32, y: i32, height: u32, direction: Direction) -> i32 {
    match direction {
        Direction::North => SIMULATION_WINDOW_HEIGHT as i32 - y,
        Direction::South => y + height as i32,
        Direction::East => x + height as i32,
        Direction::West => SIMULATION_WINDOW_WIDTH as i32 - x,
    }
}

pub fn is_past_entry_by_direction(x: i32, y: i32, height: u32, direction: Direction) -> bool {
    match direction {
        Direction::North => y <= 550,
        Direction::South => y + height as i32 >= 350,
        Direction::East => x + height as i32 >= 350,
        Direction::West => x <= 550,
    }
}

pub fn is_safe_distance_from_car(
    direction: Direction,
    x: i32,
    y: i32,
    height: i32,
    other_bb: &Rect,
    safe_distance: i32,
) -> bool {
    match direction {
        Direction::North => y >= other_bb.y() + other_bb.height() as i32 + safe_distance,
        Direction::South => y + height + safe_distance <= other_bb.y(),
        Direction::East => x + height + safe_distance <= other_bb.x(),
        Direction::West => x >= other_bb.x() + other_bb.width() as i32 + safe_distance,
    }
}

pub fn proximity_check_by_direction(
    self_bb: &Rect,
    other_bb: &Rect,
    direction: Direction,
    safe_distance: i32,
) -> bool {
    match direction {
        Direction::North => {
            let this_front = self_bb.top();
            let other_rear = other_bb.bottom();
            this_front <= other_rear + safe_distance
                && self_bb.x() < other_bb.right()
                && self_bb.right() > other_bb.x()
        }
        Direction::South => {
            let this_front = self_bb.bottom();
            let other_rear = other_bb.top();
            this_front >= other_rear - safe_distance
                && self_bb.x() < other_bb.right()
                && self_bb.right() > other_bb.x()
        }
        Direction::East => {
            let this_front = self_bb.right();
            let other_rear = other_bb.x();
            this_front >= other_rear - safe_distance
                && self_bb.y() < other_bb.bottom()
                && self_bb.bottom() > other_bb.y()
        }
        Direction::West => {
            let this_front = self_bb.x();
            let other_rear = other_bb.right();
            this_front <= other_rear + safe_distance
                && self_bb.y() < other_bb.bottom()
                && self_bb.bottom() > other_bb.y()
        }
    }
}

pub fn apply_movement_for_direction(x: &mut i32, y: &mut i32, distance: i32, direction: Direction) {
    match direction {
        Direction::North => *y -= distance,
        Direction::South => *y += distance,
        Direction::East => *x += distance,
        Direction::West => *x -= distance,
    }
}

pub fn has_exited_by_direction(x: i32, y: i32, height: u32, direction: Direction) -> bool {
    match direction {
        Direction::North => y + height as i32 <= 0,
        Direction::South => y >= SIMULATION_WINDOW_HEIGHT as i32,
        Direction::West => x + height as i32 <= 0,
        Direction::East => x >= SIMULATION_WINDOW_WIDTH as i32 + height as i32,
    }
}

pub fn calculate_speed_statistics(cars: &[Car]) -> (f32, f32, f32) {
    if cars.is_empty() {
        return (0.0, 0.0, 0.0);
    }

    let mut min_speed = f32::MAX;
    let mut max_speed = f32::MIN;
    let mut total_speed = 0.0;
    let mut valid_cars = 0;

    for car in cars {
        if let Some(exit_time) = car.time_exit {
            if let Ok(duration) = exit_time.duration_since(car.time_enter) {
                let duration_secs = duration.as_secs_f32();
                if duration_secs > 0.0 {
                    let distance = car.dist as f32;
                    let effective_speed = distance / duration_secs;
                    
                    min_speed = min_speed.min(effective_speed);
                    max_speed = max_speed.max(effective_speed);
                    total_speed += effective_speed;
                    valid_cars += 1;
                }
            }
        }
    }

    if valid_cars == 0 {
        (0.0, 0.0, 0.0)
    } else {
        let avg_speed = total_speed / valid_cars as f32;
        (round_two(min_speed), round_two(max_speed), round_two(avg_speed))
    }
}

pub fn calculate_duration_statistics(cars: &[Car]) -> (f32, f32, f32) {
    if cars.is_empty() {
        return (0.0, 0.0, 0.0);
    }

    let mut min_duration = Duration::MAX;
    let mut max_duration = Duration::ZERO;
    let mut total_duration = Duration::ZERO;
    let mut valid_cars = 0;

    for car in cars {
        if let Some(exit_time) = car.time_exit {
            if let Ok(duration) = exit_time.duration_since(car.time_enter) {
                min_duration = min_duration.min(duration);
                max_duration = max_duration.max(duration);
                total_duration += duration;
                valid_cars += 1;
            }
        }
    }

    if valid_cars == 0 {
        (0.0, 0.0, 0.0)
    } else {
        let avg_duration_secs = total_duration.as_secs_f32() / valid_cars as f32;
        (
            round_two(min_duration.as_secs_f32()),
            round_two(max_duration.as_secs_f32()),
            round_two(avg_duration_secs)
        )
    }
}

pub fn calculate_time_difference(scheduled: SystemTime, actual: SystemTime) -> f64 {
    scheduled
        .duration_since(actual)
        .map(|d| -(d.as_secs_f64()))
        .unwrap_or_else(|e| e.duration().as_secs_f64())
}

pub fn calculate_speed_from_distance_time(distance: i32, time_left: f64, default_speed: f64) -> i32 {
    if time_left > 0.0 {
        (distance as f64 / time_left).round() as i32
    } else {
        default_speed as i32
    }
}

pub fn get_color_by_state(collided: bool, brake: bool, in_intersection: bool) -> Color {
    if collided {
        Color::MAGENTA // Magenta for collision
    } else if brake {
        Color::RED // Red for braking
    } else if !in_intersection {
        Color::YELLOW // Yellow for waiting
    } else {
        Color::BLUE // Blue for active in intersection
    }
}

pub fn create_origin_rect(x: i32, y: i32, size: i32) -> Rect {
    Rect::new(
        x - size / 2,
        y - size / 2,
        size as u32,
        size as u32,
    )
}

pub fn get_position_after_turn(
    x: i32,
    y: i32,
    distance: i32,
    direction: Direction,
    turn_type: Route,
) -> (i32, i32, Direction) {
    match (direction, turn_type) {
        (Direction::North, Route::Right) => (x + distance, 558, Direction::East),
        (Direction::South, Route::Right) => (x - (CAR_HEIGHT_PX/2) as i32 - distance, 308, Direction::West),
        (Direction::East, Route::Right) => (308, y + distance, Direction::South),
        (Direction::West, Route::Right) => (558, y - (CAR_HEIGHT_PX/2) as i32 - distance, Direction::North),
        
        (Direction::North, Route::Left) => (x - (CAR_HEIGHT_PX/2) as i32 - distance, 408, Direction::West),
        (Direction::South, Route::Left) => (x + distance, 458, Direction::East),
        (Direction::East, Route::Left) => (458, y - (CAR_HEIGHT_PX/2) as i32 - distance, Direction::North),
        (Direction::West, Route::Left) => (408, y + distance, Direction::South),
        
        // Invalid combinations (should not happen)
        _ => (x, y, direction),
    }
}