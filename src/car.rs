use crate::intersection::{Direction, Route};
use chrono::{DateTime, Local};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::time::{Duration, SystemTime};

use crate::consts::*;

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

    pub fn intersects(&self, other: &Car) -> bool {
        self.bounding_box().has_intersection(other.bounding_box())
    }

    pub fn is_too_close(&self, other: &Car) -> bool {
        let safe_distance = BRAKE_DISTANCE_PX;

        let self_box = self.bounding_box();
        let other_box = other.bounding_box();

        match self.direction {
            Direction::North => {
                let this_front = self_box.top();            // my top edge
                let other_rear = other_box.bottom();        // other bottom edge
                this_front <= other_rear + safe_distance
                    && self_box.x() < other_box.right()     // horizontal overlap
                    && self_box.right() > other_box.x()
            }
            Direction::South => {
                let this_front = self_box.bottom();         // my bottom edge
                let other_rear = other_box.top();           // other top edge
                this_front >= other_rear - safe_distance
                    && self_box.x() < other_box.right()
                    && self_box.right() > other_box.x()
            }
            Direction::East => {
                let this_front = self_box.right();          // my right edge
                let other_rear = other_box.x();             // other left edge
                this_front >= other_rear - safe_distance
                    && self_box.y() < other_box.bottom()    // vertical overlap
                    && self_box.bottom() > other_box.y()
            }
            Direction::West => {
                let this_front = self_box.x();              // my left edge
                let other_rear = other_box.right();         // other right edge
                this_front <= other_rear + safe_distance
                    && self_box.y() < other_box.bottom()
                    && self_box.bottom() > other_box.y()
            }
        }
    }

    pub fn distance_to_entry(&self) -> i32 {
        match self.direction {
            Direction::North => SIMULATION_WINDOW_HEIGHT as i32 - self.y,
            Direction::South => self.y + self.height as i32,
            Direction::East => self.x + self.height as i32,
            Direction::West => SIMULATION_WINDOW_WIDTH as i32 - self.x,
        }
    }

    fn is_at_entry_boundary(&self) -> bool {
        match self.direction {
            Direction::North => self.y <= 550,
            Direction::South => self.y + self.height as i32 >= 350,
            Direction::East => self.x + self.height as i32 >= 350,
            Direction::West => self.x <= 550,
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        if self.exited || self.brake {
            return;
        }

        match self.route {
            Route::Right => self.update_right_turn(delta_time),
            Route::Left => self.update_left_turn(delta_time),
            _ => self.update_straight(delta_time),
        }

        match self.direction {
            Direction::North if self.y + self.height as i32 <= 0 => {
                self.exited = true;
            }
            Direction::South if self.y >= SIMULATION_WINDOW_HEIGHT as i32 => {
                self.exited = true;
            }
            Direction::West if self.x + self.height as i32 <= 0 => {
                self.exited = true;
            }
            Direction::East if self.x >= SIMULATION_WINDOW_WIDTH as i32 + self.height as i32 => {
                self.exited = true;
            }
            _ => {}
        }

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
            let diff = self
                .entry_time
                .duration_since(now)
                .map(|d| -(d.as_secs_f64()))
                .unwrap_or_else(|e| e.duration().as_secs_f64());

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

        let speed_px_per_sec = if time_left > 0.0 {
            distance_to_entry as f64 / time_left
        } else {
            SPEED_PX_PER_SEC
        };

        let target_speed = (speed_px_per_sec).round() as i32;

        if self.speed < target_speed {
            self.speed = (self.speed + MAX_ACCELERATION).min(target_speed).min(SPEED_PX_PER_SEC as i32);
        } else if self.speed > target_speed {
            self.speed = (self.speed - MAX_ACCELERATION).max(target_speed).max(0);
        }

        if self.route == Route::Right { self.speed = SPEED_PX_PER_SEC as i32 }
        let distance = (self.speed as f64 * seconds).round() as i32;

        match self.direction {
            Direction::North => self.y -= distance,
            Direction::South => self.y += distance,
            Direction::East  => self.x += distance,
            Direction::West  => self.x -= distance,
        }
    }

    fn update_right_turn(&mut self, delta_time: Duration) {
        let seconds = delta_time.as_secs_f64();
        let distance = (self.speed as f64 * seconds).round() as i32;
        let distance_forward = self.distance_to_entry();

        if self.turned || distance_forward < RIGHT_TURN_ENTRY_DISTANCE_PX {
            self.update_straight(delta_time);
        } else {
            match self.direction {
                Direction::North => {
                    self.direction = Direction::East;
                    self.y = 558;
                    self.x += distance;
                }
                Direction::South => {
                    self.direction = Direction::West;
                    self.y = 308;
                    self.x -= (CAR_HEIGHT_PX/2) as i32 - distance;
                }
                Direction::East => {
                    self.direction = Direction::South;
                    self.x = 308;
                    self.y += distance;
                }
                Direction::West => {
                    self.direction = Direction::North;
                    self.x = 558;
                    self.y -= (CAR_HEIGHT_PX/2) as i32 - distance;
                }
            }
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
            match self.direction {
                Direction::North => {
                    self.direction = Direction::West;
                    self.y = 408;
                    self.x -= (CAR_HEIGHT_PX/2) as i32 - distance;
                }
                Direction::South => {
                    self.direction = Direction::East;
                    self.y = 458;
                    self.x += distance;
                }
                Direction::East => {
                    self.direction = Direction::North;
                    self.x = 458;
                    self.y -= (CAR_HEIGHT_PX/2) as i32 - distance;
                }
                Direction::West => {
                    self.direction = Direction::South;
                    self.x = 408;
                    self.y += distance;
                }
            }
            self.turned = true;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let (w, h) = (self.width, self.height);

        // Offset position to account for center-based rotation
        let (offset_x, offset_y) = match self.direction {
            Direction::North | Direction::South => (0, 0),
            Direction::East | Direction::West => {
                ((h as i32 - w as i32) / 2, -(h as i32 - w as i32) / 2)
            }
        };
        let dest = Rect::new(self.x + offset_x, self.y + offset_y, w, h);
        let angle = match self.direction {
            Direction::South => 0.0,
            Direction::North => 180.0,
            Direction::East => 270.0,
            Direction::West => 90.0,
        };

        canvas
            .copy_ex(&self.texture, None, dest, angle, None, false, false)
            .unwrap();

        if self.collided {
            canvas.set_draw_color(Color::MAGENTA); // Magenta for collision
        } else if self.brake {
            canvas.set_draw_color(Color::RED); // Red for active
        } else if !self.in_intersection {
            canvas.set_draw_color(Color::YELLOW); // Yellow for waiting
        } else {
            canvas.set_draw_color(Color::BLUE); // Blue for active
        }
        canvas.draw_rect(self.bounding_box()).unwrap();

        canvas.set_draw_color(Color::BLUE);
        let origin_size = 4; // small square
        let origin = Rect::new(
            self.x - origin_size / 2,
            self.y - origin_size / 2,
            origin_size as u32,
            origin_size as u32,
        );
        canvas.fill_rect(origin).unwrap();
    }
}
