use crate::intersection::{Direction, Route};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::consts::*;

pub type ZoneIndex = (usize, usize); // e.g., (2, 1)

#[derive(Clone)]
pub struct ZoneReservation {
    pub _car_id: String,                   // Unique ID
    pub time_in: SystemTime,
    pub time_out: SystemTime,
}

pub struct CrossingManager {
    pub grid: HashMap<ZoneIndex, Vec<ZoneReservation>>,
}

impl CrossingManager {
    pub fn new() -> Self {
        let mut grid = HashMap::new();

        for y in 0..4 {
            for x in 0..4 {
                grid.insert((x, y), Vec::new());
            }
        }
        CrossingManager { grid }
    }

    pub fn latest_available_time(&self, dir: Direction, route: Route, distance_to_entry: f64) -> SystemTime {
        let path = route_to_zone_path(dir, route);
        let now = SystemTime::now();

        let zone_time = Duration::from_secs_f64(ZONE_LENGTH_PX / SPEED_PX_PER_SEC);
        let car_occupy_time = Duration::from_secs_f64(CAR_LENGTH_PX / SPEED_PX_PER_SEC);
        let safe_time_gap = Duration::from_secs_f64(SAFE_DISTANCE_PX / SPEED_PX_PER_SEC);
        let travel_time = Duration::from_secs_f64(distance_to_entry / SPEED_PX_PER_SEC);

        let mut base_time = now + travel_time;

        // Loop until we find a time with no conflicts + safety gap
        let base_time = 'try_time: loop {
            for (i, zone) in path.iter().enumerate() {
                let zone_entry_time = base_time + zone_time * i as u32;
                let zone_exit_time = zone_entry_time + car_occupy_time + safe_time_gap;

                if let Some(res_list) = self.grid.get(zone) {
                    for res in res_list {
                        let overlaps = res.time_in < zone_exit_time && res.time_out > zone_entry_time;
                        if overlaps {
                            // Conflict — delay base_time and restart
                            base_time = base_time.max(res.time_out - zone_time * i as u32);
                            continue 'try_time;
                        }
                    }
                }
            }
            break base_time;
        };
        base_time
    }

    pub fn reserve_path(&mut self, car_id: &str, dir: Direction, route: Route, distance_to_entry: f64) -> SystemTime {
        let entry_time = self.latest_available_time(dir, route, distance_to_entry);
        let path = route_to_zone_path(dir, route);
        let reservations = generate_zone_reservations(car_id, &path, entry_time);

        for (zone, reservation) in reservations {
            if let Some(zone_res_list) = self.grid.get_mut(&zone) {
                zone_res_list.push(reservation);
            }
        }

        entry_time
    }
    
    pub fn update(&mut self) {
        let now = SystemTime::now();

        for res_list in self.grid.values_mut() {
            res_list.retain(|res| res.time_out > now);
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let rect = Rect::new(300, 300, 300, 300);
        canvas.set_draw_color(Color::RGB(128, 128, 128));
        canvas.draw_rect(rect)?;

        let zone_size = ZONE_LENGTH_PX as i32;
        let start_x = 350;
        let start_y = 350;
        let now = SystemTime::now();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for y in 0..4 {
            for x in 0..4 {
                let rect = Rect::new(
                    start_x + x as i32 * zone_size,
                    start_y + y as i32 * zone_size,
                    zone_size as u32,
                    zone_size as u32,
                );

                let reservations = self.grid.get(&(y, x)).unwrap();
                let mut has_active = false;
                let mut has_any = false;

                for res in reservations {
                    has_any = true;
                    if res.time_in <= now && res.time_out > now {
                        has_active = true;
                        break;
                    }
                }

                let color = if has_active {
                    Color::RGB(160, 32, 240) // Purple (active)
                } else if has_any {
                    Color::RED    // Red (reserved, but inactive)
                } else {
                    Color::GREEN    // Green (free)
                };

                canvas.set_draw_color(color);
                canvas.draw_rect(rect)?;
            }
        }

        Ok(())
    }
}

fn route_to_zone_path(dir: Direction, route: Route) -> Vec<ZoneIndex> {
    match (dir, route) {
        (Direction::South, Route::Left) => ZONES_FOR_SOUTH_LEFT,
        (Direction::South, Route::Straight) => ZONES_FOR_SOUTH_STRAIGHT,

        (Direction::North, Route::Left) => ZONES_FOR_NORTH_LEFT,
        (Direction::North, Route::Straight) => ZONES_FOR_NORTH_STRAIGHT,

        (Direction::East, Route::Left)  => ZONES_FOR_EAST_LEFT,
        (Direction::East, Route::Straight) => ZONES_FOR_EAST_STRAIGHT,

        (Direction::West, Route::Left) => ZONES_FOR_WEST_LEFT,
        (Direction::West, Route::Straight) => ZONES_FOR_WEST_STRAIGHT,

        (_, Route::Right) => vec![],
    }
}

pub fn generate_zone_reservations(
    car_id: &str,
    path: &[ZoneIndex],
    entry_time: SystemTime,
) -> Vec<(ZoneIndex, ZoneReservation)> {
    let zone_time = Duration::from_secs_f64(ZONE_LENGTH_PX / SPEED_PX_PER_SEC);
    let occupy_time = Duration::from_secs_f64(CAR_LENGTH_PX / SPEED_PX_PER_SEC);
    let safe_gap = Duration::from_secs_f64(SAFE_DISTANCE_PX / SPEED_PX_PER_SEC);

    let mut reservations = Vec::new();

    for (i, &zone) in path.iter().enumerate() {
        let time_in = entry_time + zone_time * i as u32;
        let time_out = time_in + occupy_time + safe_gap;

        let reservation = ZoneReservation {
            _car_id: car_id.to_string(),
            time_in,
            time_out,
        };

        reservations.push((zone, reservation));
    }

    reservations
}
