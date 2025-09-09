use crate::intersection::{Direction, Route};

pub struct CarIdGenerator {
    current: usize,
    max: usize,
}

impl CarIdGenerator {
    pub fn new() -> Self {
        CarIdGenerator {
            current: 0,
            max: 9999,
        }
    }

    pub fn get_next(&mut self, direction: Direction, route: Route) -> String {
        let prefix = match (direction, route) {
            (Direction::South, Route::Right)    => "SRT",
            (Direction::South, Route::Left)     => "SLT",
            (Direction::South, Route::Straight) => "SST",
            (Direction::North, Route::Right)    => "NRT",
            (Direction::North, Route::Left)     => "NLT",
            (Direction::North, Route::Straight) => "NST",
            (Direction::East,  Route::Right)    => "ERT",
            (Direction::East,  Route::Left)     => "ELT",
            (Direction::East,  Route::Straight) => "EST",
            (Direction::West,  Route::Right)    => "WRT",
            (Direction::West,  Route::Left)     => "WLT",
            (Direction::West,  Route::Straight) => "WST",
        };
        format!("{}-{:04}", prefix, self.next().unwrap())
    }

    pub fn next(&mut self) -> Option<usize> {
        if self.current >= self.max {
            None // Stop at 9999
        } else {
            self.current += 1;
            Some(self.current)
        }
    }
}
