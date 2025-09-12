use sdl2::render::{Canvas, Texture};
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::video::Window;
use crate::intersection::{Route, Direction};
use std::time::{SystemTime, Duration};
use chrono::{DateTime, Local};

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

    pub time_enter: SystemTime,
    pub time_exit: Option<SystemTime>,
    pub entry_time: SystemTime,
    pub in_intersection: bool,
    pub dist: i32,
    pub actual_entry_time: Option<SystemTime>,
}

impl<'b> Car<'b> {
    const MAX_SPEED: i32 = 5;
    const ENTRY_DISTANCE_PX: i32 = 350;

    pub fn new(
        id: String, x: i32, y: i32, width: u32, height: u32,
        speed: i32, texture: &'b Texture<'b>,
        route: Route, entry_time: SystemTime, direction: Direction,
    ) -> Self {
        let dist = match route {
            Route::Right => 650,
            Route::Straight => 900,
            Route::Left => 950,
        };

        Car {
            id, x, y, width, height,
            speed, texture,
            route, direction, turned: false, exited: false, collided: false,
            time_enter: SystemTime::now(), time_exit: None, entry_time, in_intersection: false,
            dist, actual_entry_time: None,
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

    pub fn distance_to_entry(&self) -> i32 {
        match self.direction {
            Direction::North => 900 - self.y,
            Direction::South => self.y + self.height as i32,
            Direction::East  => self.x + self.height as i32,
            Direction::West  => 900 - self.x,
        }
    }

    fn is_at_entry_boundary(&self) -> bool {
        match self.direction {
            Direction::North => self.y <= 550,
            Direction::South => self.y + self.height as i32 >= 350,
            Direction::East  => self.x + self.height as i32 >= 350,
            Direction::West  => self.x <= 550,
        }
    }
    pub fn update(&mut self) {
        if self.exited {
            return;
        }

        match self.route {
            Route::Right => self.update_right_turn(),
            Route::Left => self.update_left_turn(),
            _ => self.update_straight(),
        }

        match self.direction {
            Direction::North if self.y + self.height as i32 <= 0 => {
                self.exited = true;
            }
            Direction::South if self.y >= 900 => {
                self.exited = true;
            }
            Direction::West if self.x + self.height as i32 <= 0 => {
                self.exited = true;
            }
            Direction::East if self.x >= 900 + self.height as i32 => {
                self.exited = true;
            }
            _ => {}
        }

        if self.exited { self.time_exit = Some(SystemTime::now()); }
    }

    fn update_straight(&mut self) {
        let now = SystemTime::now();

        if !self.in_intersection && now < self.entry_time {
            if self.is_at_entry_boundary() {
                // Stop here until allowed
                return;
            }
        }

        if !self.in_intersection && self.is_at_entry_boundary() {
            self.in_intersection = true;
            self.actual_entry_time = Some(now);

            let actual_dt: DateTime<Local> = now.into();
            let scheduled_dt: DateTime<Local> = self.entry_time.into();
            let diff = self.entry_time.duration_since(now)
                .map(|d| -(d.as_secs_f64()))
                .unwrap_or_else(|e| e.duration().as_secs_f64());

            println!(
                "🚗 Car {} ENTERED at {}, scheduled: {}, diff: {:.3}s",
                self.id,
                actual_dt.format("%H:%M:%S%.3f"),
                scheduled_dt.format("%H:%M:%S%.3f"),
                diff,
            );
        }

        let distance_to_entry = (Self::ENTRY_DISTANCE_PX - self.distance_to_entry()).max(0);
        let time_left = self.entry_time
            .duration_since(now)
            .unwrap_or(Duration::ZERO)
            .as_secs_f64();

        let speed_px_per_sec = if time_left > 0.0 {
            distance_to_entry as f64 / time_left
        } else {
            (Self::MAX_SPEED * 60) as f64
        };

        let target_speed = (speed_px_per_sec / 60.0).round() as i32;
        let max_acceleration = 1;

        if self.speed < target_speed {
            self.speed = (self.speed + max_acceleration).min(target_speed).min(Self::MAX_SPEED);
        } else if self.speed > target_speed {
            self.speed = (self.speed - max_acceleration).max(target_speed).max(0);
        }

        if self.route == Route::Right { self.speed = 7 }

        match self.direction {
            Direction::North => self.y -= self.speed,
            Direction::South => self.y += self.speed,
            Direction::East  => self.x += self.speed,
            Direction::West  => self.x -= self.speed,
        }
    }

    fn update_right_turn(&mut self) {
        let distance_forward = self.distance_to_entry();

        if self.turned || distance_forward < 350 {
            self.update_straight();
        } else {
            match self.direction {
                Direction::North => {
                    self.direction = Direction::East;
                    self.y = 558;
                    self.x += self.speed;
                }
                Direction::South => {
                    self.direction = Direction::West;
                    self.y = 308;
                    self.x -= 40 -self.speed;
                }
                Direction::East => {
                    self.direction = Direction::South;
                    self.x = 308;
                    self.y += self.speed;
                }
                Direction::West => {
                    self.direction = Direction::North;
                    self.x = 558;
                    self.y -= 40 - self.speed;
                }
            }
            self.turned = true;
        }
    }

    
    fn update_left_turn(&mut self) {
        let distance_forward = self.distance_to_entry();

        if self.turned || distance_forward < 500 {
            self.update_straight();
        } else {
            match self.direction {
                Direction::North => {
                    self.direction = Direction::West;
                    self.y = 408;
                    self.x -= 40 - self.speed;
                }
                Direction::South => {
                    self.direction = Direction::East;
                    self.y = 458;
                    self.x += self.speed;
                }
                Direction::East => {
                    self.direction = Direction::North;
                    self.x = 458;
                    self.y -= 40 - self.speed;
                }
                Direction::West => {
                    self.direction = Direction::South;
                    self.x = 408;
                    self.y += self.speed;
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
            Direction::East | Direction::West => ((h as i32 - w as i32) / 2, -(h as i32 - w as i32) / 2),
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
            
        if !self.in_intersection {
            canvas.set_draw_color(Color::RGB(255, 255, 0)); // Yellow for waiting
        } else {
            canvas.set_draw_color(Color::RGB(0, 0, 255)); // Green for active
        }
        canvas.draw_rect(self.bounding_box()).unwrap();

        canvas.set_draw_color(Color::BLUE);
        let origin_size = 4; // small square
        let origin = Rect::new(self.x - origin_size / 2, self.y - origin_size / 2, origin_size as u32, origin_size as u32);
        canvas.fill_rect(origin).unwrap();
    }
}