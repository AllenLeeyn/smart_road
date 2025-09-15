use crate::crossing_manager::ZoneIndex;
use sdl2::pixels::Color;
use std::time::Duration;

pub const BASE_DELTA_TIME: Duration = Duration::from_millis(16);

pub const MAX_SPEED: i32 = 5;
pub const BRAKE_DISTANCE_PX: i32 = 10;

// pub const RIGHT_TURN_SPEED_PX: i32 = 7;
pub const ENTRY_DISTANCE_PX: i32 = 350;
pub const ENTRY_DISTANCE_PX_RIGHT: i32 = 300;
pub const LEFT_TURN_ENTRY_DISTANCE_PX: i32 = 500;
pub const RIGHT_TURN_ENTRY_DISTANCE_PX: i32 = 350;

pub const ZONE_LENGTH_PX: f64 = 50.0;
pub const SPEED_PX_PER_SEC: f64 = MAX_SPEED as f64 * 60.0;
pub const SAFE_DISTANCE_PX: f64 = CAR_HEIGHT_PX as f64 / 2.0;

pub const CAR_WIDTH_PX: u32 = 33;
pub const CAR_HEIGHT_PX: u32 = 78;

pub const SPAWN_POSITION_SOUTH_LEFT: (i32, i32, i32) = (408, -80, 5);
pub const SPAWN_POSITION_SOUTH_STRAIGHT: (i32, i32, i32) = (358, -80, 5);
pub const SPAWN_POSITION_SOUTH_RIGHT: (i32, i32, i32) = (308, -80, 7);

pub const SPAWN_POSITION_NORTH_LEFT: (i32, i32, i32) = (458, 900, 5);
pub const SPAWN_POSITION_NORTH_STRAIGHT: (i32, i32, i32) = (508, 900, 5);
pub const SPAWN_POSITION_NORTH_RIGHT: (i32, i32, i32) = (558, 900, 7);

pub const SPAWN_POSITION_EAST_LEFT: (i32, i32, i32) = (-80, 458, 5);
pub const SPAWN_POSITION_EAST_STRAIGHT: (i32, i32, i32) = (-80, 508, 5);
pub const SPAWN_POSITION_EAST_RIGHT: (i32, i32, i32) = (-80, 558, 7);

pub const SPAWN_POSITION_WEST_LEFT: (i32, i32, i32) = (900, 408, 5);
pub const SPAWN_POSITION_WEST_STRAIGHT: (i32, i32, i32) = (900, 358, 5);
pub const SPAWN_POSITION_WEST_RIGHT: (i32, i32, i32) = (900, 308, 7);

pub const ZONES_FOR_SOUTH_LEFT: &[ZoneIndex] = &[(0, 1), (1, 1), (2, 1), (2, 2), (2, 3)];
pub const ZONES_FOR_SOUTH_STRAIGHT: &[ZoneIndex] = &[(0, 0), (1, 0), (2, 0), (3, 0)];

pub const ZONES_FOR_NORTH_LEFT: &[ZoneIndex] = &[(3, 2), (2, 2), (1, 2), (1, 1), (1, 0)];
pub const ZONES_FOR_NORTH_STRAIGHT: &[ZoneIndex] = &[(3, 3), (2, 3), (1, 3), (0, 3)];

pub const ZONES_FOR_EAST_LEFT: &[ZoneIndex] = &[(2, 0), (2, 1), (2, 2), (1, 2), (0, 2)];
pub const ZONES_FOR_EAST_STRAIGHT: &[ZoneIndex] = &[(3, 0), (3, 1), (3, 2), (3, 3)];

pub const ZONES_FOR_WEST_LEFT: &[ZoneIndex] = &[(1, 3), (1, 2), (1, 1), (2, 1), (3, 1)];
pub const ZONES_FOR_WEST_STRAIGHT: &[ZoneIndex] = &[(0, 3), (0, 2), (0, 1), (0, 0)];

pub const SIMULATION_WINDOW_WIDTH: u32 = 900;
pub const SIMULATION_WINDOW_HEIGHT: u32 = 900;
pub const STATS_WINDOW_WIDTH: u32 = 400;
pub const STATS_WINDOW_HEIGHT: u32 = 400;
pub const FONT_SIZE: u16 = 20;
pub const UI_PADDING_X: i32 = 10;
pub const UI_PADDING_Y: i32 = 10;

pub const BACKGROUND_COLOR: Color = Color::RGB(30, 30, 30);
pub const INTERSECTION_COLOR: Color = Color::RGB(128, 128, 128);

pub const MAX_ACCELERATION: i32 = 30;

pub const ROUTE_RIGHT_DISTANCE: i32 = 650;
pub const ROUTE_STRAIGHT_DISTANCE: i32 = 900;
pub const ROUTE_LEFT_DISTANCE: i32 = 950;

pub const INTERSECTION_RECT_SIZE: i32 = 300;
pub const INTERSECTION_START_X: i32 = 350;
pub const INTERSECTION_START_Y: i32 = 350;