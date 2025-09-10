use sdl2::render::{Canvas, Texture};
use sdl2::rect::Rect;
use sdl2::video::Window;
use crate::intersection::{Route, Direction};
use std::time::{SystemTime, Duration};

pub struct Car<'a> {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub speed: i32,
    pub texture: &'a Texture<'a>,
    pub route: Route,
    pub direction: Direction,
    pub turned: bool,
    pub exited: bool,

    pub time_enter: SystemTime,
    pub time_exit: Option<SystemTime>,
    pub entry_time: SystemTime,
    pub dist: i32,
}

impl<'a> Car<'a> {
    const MAX_SPEED: i32 = 7;
    const ENTRY_DISTANCE_PX: i32 = 350;

    pub fn new(
        id: String, x: i32, y: i32, width: u32, height: u32,
        speed: i32, texture: &'a Texture<'a>,
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
            route, direction, turned: false, exited: false,
            time_enter: SystemTime::now(), time_exit: None, entry_time,
            dist
        }
    }

    pub fn distance_to_entry(&self) -> i32 {
        match self.direction {
            Direction::North => 900 - self.y - 60,
            Direction::South => self.y,
            Direction::East  => self.x,
            Direction::West  => 900 - self.x - 40,
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
        let distance_to_entry = (self.distance_to_entry() - Self::ENTRY_DISTANCE_PX).max(0);

        let time_left = self.entry_time
            .duration_since(now)
            .unwrap_or(Duration::ZERO)
            .as_secs_f64();

        let speed_px_per_sec = if time_left > 0.0 {
            distance_to_entry as f64 / time_left
        } else {
            (Self::MAX_SPEED * 60) as f64  // Max speed in px/s
        };

        // Convert target speed from px/sec to px/frame
        let target_speed = (speed_px_per_sec / 60.0).round() as i32;

        // Gradually accelerate/decelerate toward target_speed
        let max_acceleration = 1; // px/frame² (change per frame)
        if self.speed < target_speed {
            self.speed = (self.speed + max_acceleration).min(target_speed).min(Self::MAX_SPEED);
        } else if self.speed > target_speed {
            self.speed = (self.speed - max_acceleration).max(target_speed).max(1);
        }

            match self.direction {
                Direction::North => self.y -= self.speed,
                Direction::South => self.y += self.speed,
                Direction::East  => self.x += self.speed,
                Direction::West  => self.x -= self.speed,
            }
        }

    fn update_right_turn(&mut self) {
        let distance_forward = self.distance_to_entry();

        if self.turned || distance_forward < 300 {
            self.update_straight();
        } else {
            match self.direction {
                Direction::North => {
                    self.direction = Direction::East;
                    self.y = 538;
                    self.x += self.speed;
                }
                Direction::South => {
                    self.direction = Direction::West;
                    self.y = 288;
                    self.x -= self.speed;
                }
                Direction::East => {
                    self.direction = Direction::South;
                    self.x = 308;
                    self.y += self.speed;
                }
                Direction::West => {
                    self.direction = Direction::North;
                    self.x = 558;
                    self.y -= self.speed;
                }
            }
            self.turned = true;
        }
    }

    
    fn update_left_turn(&mut self) {
        let distance_forward = self.distance_to_entry();

        if self.turned || distance_forward < 450 {
            self.update_straight();
        } else {
            match self.direction {
                Direction::North => {
                    self.direction = Direction::West;
                    self.y = 388;
                    self.x += self.speed;
                }
                Direction::South => {
                    self.direction = Direction::East;
                    self.y = 438;
                    self.x -= self.speed;
                }
                Direction::East => {
                    self.direction = Direction::North;
                    self.x = 458;
                    self.y += self.speed;
                }
                Direction::West => {
                    self.direction = Direction::South;
                    self.x = 408;
                    self.y -= self.speed;
                }
            }
            self.turned = true;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let dest = Rect::new(self.x, self.y, self.width, self.height);
        let angle = match self.direction {
            Direction::South => 0.0,
            Direction::North => 180.0,
            Direction::East => 270.0,
            Direction::West => 90.0,
        };

        canvas
            .copy_ex(&self.texture, None, dest, angle, None, false, false)
            .unwrap();
    }
}