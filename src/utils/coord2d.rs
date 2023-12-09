use std::cmp;
use std::fmt::Display;
use std::ops;
use std::str::FromStr;

use itertools::Itertools;

use crate::{AoCError, Coordinate, Direction};
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Coord2D {
    pub x: i32,
    pub y: i32,
}

impl Coordinate for Coord2D {
    const MAX: Self = Self {
        x: i32::MAX,
        y: i32::MAX,
    };
    const MIN: Self = Self {
        x: i32::MIN,
        y: i32::MIN,
    };

    fn distance(&self, other: &Self) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    fn get_max(&self, other: &Self) -> Self {
        Self {
            x: cmp::max(self.x, other.x),
            y: cmp::max(self.y, other.y),
        }
    }

    fn get_min(&self, other: &Self) -> Self {
        Self {
            x: cmp::min(self.x, other.x),
            y: cmp::min(self.y, other.y),
        }
    }

    fn is_empty(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn len(&self) -> f64 {
        (self.len_sqr() as f64).sqrt()
    }

    fn len_sqr(&self) -> i64 {
        self.x.pow(2) as i64 + self.y.pow(2) as i64
    }

    fn len_manhatten(&self) -> u32 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

impl FromStr for Coord2D {
    type Err = AoCError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split(',')
            .map(|c| c.parse().map_err(AoCError::new_from_parseerror))
            .collect_tuple()
            .ok_or_else(|| {
                AoCError::new(format!(
                    "String '{s}' has {} commas, expected 1",
                    s.chars().filter(|c| *c == ',').count()
                ))
            })?;

        Ok(Self { x: x?, y: y? })
    }
}

impl ops::Add for Coord2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Coord2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Coord2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Coord2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul for Coord2D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::MulAssign for Coord2D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: num::NumCast> ops::Mul<T> for Coord2D {
    type Output = Option<Self>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: i32 = num::NumCast::from(rhs)?;
        Some(Self {
            x: rhs * self.x,
            y: rhs * self.y,
        })
    }
}

impl<T: num::NumCast> ops::Div<T> for Coord2D {
    type Output = Option<Self>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: f64 = num::NumCast::from(rhs)?;
        Some(Self {
            x: ((self.x as f64) / rhs).floor() as i32,
            y: ((self.y as f64) / rhs).floor() as i32,
        })
    }
}

impl ops::Neg for Coord2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
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
        Self { x: tup.0, y: tup.1 }
    }
}

impl From<Direction> for Coord2D {
    fn from(dir: Direction) -> Self {
        dir.to_coord()
    }
}
