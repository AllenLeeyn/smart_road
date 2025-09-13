pub const MAX_SPEED: i32 = 5;
pub const BRAKE_DISTANCE_PX: i32 = 20;

pub const RIGHT_TURN_SPEED_PX: i32 = 7;
pub const ENTRY_DISTANCE_PX: i32 = 350;
pub const ENTRY_DISTANCE_PX_RIGHT: i32 = 300;
pub const LEFT_TURN_ENTRY_DISTANCE_PX: i32 = 500;
pub const RIGHT_TURN_ENTRY_DISTANCE_PX: i32 = 350;

pub const ZONE_LENGTH_PX: f64 = 50.0;
pub const CAR_LENGTH_PX: f64 = 78.0;
pub const SPEED_PX_PER_SEC: f64 = 300.0;
pub const SAFE_DISTANCE_PX: f64 = 50.0 + (5.0 * 15.0);

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


pub const ZONES_FOR_SOUTH_LEFT: Vec<ZoneIndex> = vec![(0,1), (1,1), (2,1), (2,2), (2,3)];
pub const ZONES_FOR_SOUTH_STRAIGHT: Vec<ZoneIndex> = vec![(0,0), (1,0), (2,0), (3,0)];

pub const ZONES_FOR_NORTH_LEFT: Vec<ZoneIndex> = vec![(3,2), (2,2), (1,2), (1,1), (1,0)];
pub const ZONES_FOR_NORTH_STRAIGHT: Vec<ZoneIndex> = vec![(3,3), (2,3), (1,3), (0,3)];

pub const ZONES_FOR_EAST_LEFT: Vec<ZoneIndex> = vec![(2,0), (2,1), (2,2), (1,2), (0,2)];
pub const ZONES_FOR_EAST_STRAIGHT: Vec<ZoneIndex> = vec![(3,0), (3,1), (3,2), (3,3)];

pub const ZONES_FOR_WEST_LEFT: Vec<ZoneIndex> = vec![(1,3), (1,2), (1,1), (2,1), (3,1)];
pub const ZONES_FOR_WEST_STRAIGHT: Vec<ZoneIndex> = vec![(0,3), (0,2), (0,1), (0,0)];