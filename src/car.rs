use crate::intersection::{Direction, Route};
use chrono::{DateTime, Local};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::time::{Duration, SystemTime};

use crate::consts::*;
use crate::utils::*;

pub struct Car<'b> {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub speed: i32,
    pub texture: &'b Texture<'b>,
    pub route: Route,
    pub direction: Direction,
    pub turned: bool,
    pub exited: bool,
    pub collided: bool,
    pub brake: bool,

    pub time_enter: SystemTime,
    pub time_exit: Option<SystemTime>,
    pub entry_time: SystemTime,
    pub in_intersection: bool,
    pub dist: i32,
    pub actual_entry_time: Option<SystemTime>,
}

impl<'b> Car<'b> {
    pub fn new(
        id: String, x: i32, y: i32, width: u32, height: u32,
        speed: i32, texture: &'b Texture<'b>,
        route: Route, entry_time: SystemTime, direction: Direction,
    ) -> Self {
        let dist = match route {
            Route::Right => ROUTE_RIGHT_DISTANCE,
            Route::Straight => ROUTE_STRAIGHT_DISTANCE,
            Route::Left => ROUTE_LEFT_DISTANCE,
        };

        Car {
            id,
            x,
            y,
            width,
            height,
            speed,
            texture,
            route,
            direction,
            turned: false,
            exited: false,
            collided: false,
            time_enter: SystemTime::now(),
            time_exit: None,
            entry_time,
            in_intersection: false,
            dist,
            actual_entry_time: None,
            brake: false,
        }
    }

    pub fn bounding_box(&self) -> Rect {
        let (w, h) = match self.direction {
            Direction::North | Direction::South => (self.width, self.height),
            Direction::East | Direction::West => (self.height, self.width),
        };

        Rect::new(self.x, self.y, w, h)
    }

    pub fn intersects(&self, other: &Car) -> bool { self.bounding_box().has_intersection(other.bounding_box()) }

    pub fn is_too_close(&self, other: &Car) -> bool {
        let safe_distance = BRAKE_DISTANCE_PX;

        let self_box = self.bounding_box();
        let other_box = other.bounding_box();

        proximity_check_by_direction(&self_box, &other_box, self.direction, safe_distance)
    }

    pub fn distance_to_entry(&self) -> i32 { get_distance_by_direction(self.x, self.y, self.height, self.direction) }

    fn is_at_entry_boundary(&self) -> bool { is_past_entry_by_direction(self.x, self.y, self.height, self.direction) }

    pub fn update(&mut self, delta_time: Duration) {
        if self.exited || self.brake {
            return;
        }

        match self.route {
            Route::Right => self.update_right_turn(delta_time),
            Route::Left => self.update_left_turn(delta_time),
            _ => self.update_straight(delta_time),
        }

        self.exited = has_exited_by_direction(self.x, self.y, self.height, self.direction);

        if self.exited {
            self.time_exit = Some(SystemTime::now());
        }
    }

    fn update_straight(&mut self, delta_time: Duration) {
        let now = SystemTime::now();
        let seconds = delta_time.as_secs_f64();

        if !self.in_intersection && self.is_at_entry_boundary() {
            self.in_intersection = true;
            self.actual_entry_time = Some(now);

            let actual_dt: DateTime<Local> = now.into();
            let scheduled_dt: DateTime<Local> = self.entry_time.into();
            let diff = calculate_time_difference(self.entry_time, now);

            println!(
                "Car {} ENTERED at {}, scheduled: {}, diff: {:.3}s",
                self.id,
                actual_dt.format("%H:%M:%S%.3f"),
                scheduled_dt.format("%H:%M:%S%.3f"),
                diff,
            );
        }

        let distance_to_entry = (ENTRY_DISTANCE_PX - self.distance_to_entry()).max(0);
        let time_left = self
            .entry_time
            .duration_since(now)
            .unwrap_or(Duration::ZERO)
            .as_secs_f64();

        let target_speed = calculate_speed_from_distance_time(
            distance_to_entry,
            time_left,
            SPEED_PX_PER_SEC
        );

        if self.speed < target_speed {
            self.speed = (self.speed + MAX_ACCELERATION).min(target_speed).min(SPEED_PX_PER_SEC as i32);
        } else if self.speed > target_speed {
            self.speed = (self.speed - MAX_ACCELERATION).max(target_speed).max(0);
        }

        if self.route == Route::Right { self.speed = SPEED_PX_PER_SEC as i32 }
        let distance = (self.speed as f64 * seconds).round() as i32;

        apply_movement_for_direction(&mut self.x, &mut self.y, distance, self.direction);
    }

    fn update_right_turn(&mut self, delta_time: Duration) {
        let seconds = delta_time.as_secs_f64();
        let distance = (self.speed as f64 * seconds).round() as i32;
        let distance_forward = self.distance_to_entry();

        if self.turned || distance_forward < RIGHT_TURN_ENTRY_DISTANCE_PX {
            self.update_straight(delta_time);
        } else {
            let (new_x, new_y, new_direction) = get_position_after_turn(
                self.x, self.y, distance, self.direction, Route::Right
            );
            self.x = new_x;
            self.y = new_y;
            self.direction = new_direction;
            self.turned = true;
        }
    }

    fn update_left_turn(&mut self, delta_time: Duration) {
        let seconds = delta_time.as_secs_f64();
        let distance = (self.speed as f64 * seconds).round() as i32;
        let distance_forward = self.distance_to_entry();

        if self.turned || distance_forward < LEFT_TURN_ENTRY_DISTANCE_PX {
            self.update_straight(delta_time);
        } else {
            let (new_x, new_y, new_direction) = get_position_after_turn(
                self.x, self.y, distance, self.direction, Route::Left
            );
            self.x = new_x;
            self.y = new_y;
            self.direction = new_direction;
            self.turned = true;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let (w, h) = (self.width, self.height);

        // Offset position to account for center-based rotation
        let (offset_x, offset_y) = get_direction_offset_by_direction(self.direction, w, h);
        let dest = Rect::new(self.x + offset_x, self.y + offset_y, w, h);
        let angle = get_angle_by_direction(self.direction);

        canvas
            .copy_ex(&self.texture, None, dest, angle, None, false, false)
            .unwrap();

        let state_color = get_color_by_state(self.collided, self.brake, self.in_intersection);
        canvas.set_draw_color(state_color);
        canvas.draw_rect(self.bounding_box()).unwrap();

        canvas.set_draw_color(Color::BLUE);
        let origin = create_origin_rect(self.x, self.y, 4);
        canvas.fill_rect(origin).unwrap();
    }
}
