use std::cmp;
use std::fmt::Display;
use std::ops;

use crate::Direction;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Coord2D {
    pub x: i32,
    pub y: i32,
}

impl Coord2D {
    pub const MAX: Coord2D = Coord2D {
        x: i32::MAX,
        y: i32::MAX,
    };
    pub const MIN: Coord2D = Coord2D {
        x: i32::MIN,
        y: i32::MIN,
    };

    pub fn parse(data: &str) -> Coord2D {
        let (x, y) = data
            .split_once(',')
            .expect("Coordinates must be in the form x,y");
        Coord2D {
            x: x.parse().expect("Failed to parse X:"),
            y: y.parse().expect("Failed to parse Y:"),
        }
    }

    pub fn distance(&self, other: &Coord2D) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    pub fn get_max(&self, other: &Coord2D) -> Coord2D {
        Coord2D {
            x: cmp::max(self.x, other.x),
            y: cmp::max(self.y, other.y),
        }
    }

    pub fn get_min(&self, other: &Coord2D) -> Coord2D {
        Coord2D {
            x: cmp::min(self.x, other.x),
            y: cmp::min(self.y, other.y),
        }
    }

    pub fn len(&self) -> f64 {
        (self.len_sqr() as f64).sqrt()
    }

    pub fn len_sqr(&self) -> i64 {
        self.x.pow(2) as i64 + self.y.pow(2) as i64
    }

    pub fn len_manhatten(&self) -> u32 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

impl ops::Add for Coord2D {
    type Output = Self;
    fn add(self, rhs: Coord2D) -> Self::Output {
        Coord2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Coord2D {
    type Output = Self;
    fn sub(self, rhs: Coord2D) -> Self::Output {
        Coord2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul for Coord2D {
    type Output = Self;
    fn mul(self, rhs: Coord2D) -> Self::Output {
        Coord2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: num::NumCast> ops::Mul<T> for Coord2D {
    type Output = Option<Self>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: i32 = num::NumCast::from(rhs)?;
        Some(Coord2D {
            x: rhs * self.x,
            y: rhs * self.y,
        })
    }
}

impl<T: num::NumCast> ops::Div<T> for Coord2D {
    type Output = Option<Self>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: f64 = num::NumCast::from(rhs)?;
        Some(Coord2D {
            x: ((self.x as f64) / rhs).floor() as i32,
            y: ((self.y as f64) / rhs).floor() as i32,
        })
    }
}

impl ops::Neg for Coord2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Coord2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Coord2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        self.x.fmt(f)?;
        write!(f, ",")?;
        self.y.fmt(f)?;
        write!(f, "]")
    }
}

impl From<(i32, i32)> for Coord2D {
    fn from(tup: (i32, i32)) -> Self {
        Coord2D { x: tup.0, y: tup.1 }
    }
}

impl From<Direction> for Coord2D {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => Coord2D { x: 0, y: 1 },
            Direction::East => Coord2D { x: 1, y: 0 },
            Direction::South => Coord2D { x: 0, y: -1 },
            Direction::West => Coord2D { x: -1, y: 0 },
        }
    }
}
