use std::cmp;
use std::fmt::Display;
use std::ops;
use std::str::FromStr;

use itertools::Itertools;

use crate::{AoCError, Coordinate, Direction};
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct BigCoord2D {
    pub x: i64,
    pub y: i64,
}

impl Coordinate for BigCoord2D {
    type UnsignedLen = u64;
    type SignedLen = i128;

    const MAX: Self = Self {
        x: i64::MAX,
        y: i64::MAX,
    };
    const MIN: Self = Self {
        x: i64::MIN,
        y: i64::MIN,
    };

    fn distance(&self, other: &Self) -> Self::UnsignedLen {
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

    fn len_sqr(&self) -> Self::SignedLen {
        self.x.pow(2) as Self::SignedLen + self.y.pow(2) as Self::SignedLen
    }

    fn len_manhatten(&self) -> Self::UnsignedLen {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

impl FromStr for BigCoord2D {
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

impl ops::Add for BigCoord2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for BigCoord2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for BigCoord2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for BigCoord2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul for BigCoord2D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::MulAssign for BigCoord2D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: num::NumCast> ops::Mul<T> for BigCoord2D {
    type Output = Option<Self>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: i64 = num::NumCast::from(rhs)?;
        Some(Self {
            x: rhs * self.x,
            y: rhs * self.y,
        })
    }
}

impl<T: num::NumCast> ops::Div<T> for BigCoord2D {
    type Output = Option<Self>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: f64 = num::NumCast::from(rhs)?;
        Some(Self {
            x: ((self.x as f64) / rhs).floor() as i64,
            y: ((self.y as f64) / rhs).floor() as i64,
        })
    }
}

impl ops::Neg for BigCoord2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for BigCoord2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        self.x.fmt(f)?;
        write!(f, ",")?;
        self.y.fmt(f)?;
        write!(f, "]")
    }
}

impl From<(i32, i32)> for BigCoord2D {
    fn from((x, y): (i32, i32)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl From<(u32, u32)> for BigCoord2D {
    fn from((x, y): (u32, u32)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl From<(i64, i64)> for BigCoord2D {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl TryFrom<(u64, u64)> for BigCoord2D {
    type Error = std::num::TryFromIntError;

    fn try_from((x, y): (u64, u64)) -> Result<Self, Self::Error> {
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
        })
    }
}

impl TryFrom<(usize, usize)> for BigCoord2D {
    type Error = std::num::TryFromIntError;

    fn try_from((x, y): (usize, usize)) -> Result<Self, Self::Error> {
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
        })
    }
}

impl From<Direction> for BigCoord2D {
    fn from(dir: Direction) -> Self {
        let c = dir.to_coord();
        (c.x, c.y).into()
    }
}
