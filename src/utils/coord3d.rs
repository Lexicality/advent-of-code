// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;
use std::ops;
use std::str::FromStr;

use itertools::Itertools;

use crate::{AoCError, Coordinate};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Coord3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coordinate for Coord3D {
    type Type = i32;

    const MAX: Self = Self {
        x: i32::MAX,
        y: i32::MAX,
        z: i32::MAX,
    };
    const MIN: Self = Self {
        x: i32::MIN,
        y: i32::MIN,
        z: i32::MIN,
    };

    fn distance(&self, other: &Self) -> Self::Type {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn get_max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    fn get_min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    fn is_empty(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
    }

    fn len(&self) -> f64 {
        (self.len_sqr() as f64).sqrt()
    }

    fn len_sqr(&self) -> Self::Type {
        self.x.pow(2) + self.y.pow(2) + self.z.pow(2)
    }

    fn len_manhatten(&self) -> Self::Type {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
type CoordType = <Coord3D as Coordinate>::Type;

impl FromStr for Coord3D {
    type Err = AoCError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .map(|c| c.parse())
            .collect_tuple()
            .ok_or_else(|| {
                AoCError::new(format!(
                    "String '{s}' has {} commas, expected 2",
                    s.chars().filter(|c| *c == ',').count()
                ))
            })?;

        Ok(Self {
            x: x?,
            y: y?,
            z: z?,
        })
    }
}

impl ops::Add for Coord3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Coord3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub for Coord3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Coord3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul for Coord3D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::MulAssign for Coord3D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: num::NumCast> ops::Mul<T> for Coord3D {
    type Output = Option<Self>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: CoordType = num::NumCast::from(rhs)?;
        Some(Self {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        })
    }
}

impl<T: num::NumCast> ops::Div<T> for Coord3D {
    type Output = Option<Self>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: f64 = num::NumCast::from(rhs)?;
        Some(Self {
            x: ((self.x as f64) / rhs).floor() as CoordType,
            y: ((self.y as f64) / rhs).floor() as CoordType,
            z: ((self.z as f64) / rhs).floor() as CoordType,
        })
    }
}

impl ops::Neg for Coord3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for Coord3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        self.x.fmt(f)?;
        write!(f, ",")?;
        self.y.fmt(f)?;
        write!(f, ",")?;
        self.z.fmt(f)?;
        write!(f, "]")
    }
}

impl From<(i32, i32, i32)> for Coord3D {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }
}

impl TryFrom<(u32, u32, u32)> for Coord3D {
    type Error = std::num::TryFromIntError;

    fn try_from((x, y, z): (u32, u32, u32)) -> Result<Self, Self::Error> {
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
            z: z.try_into()?,
        })
    }
}

impl TryFrom<(usize, usize, usize)> for Coord3D {
    type Error = std::num::TryFromIntError;

    fn try_from((x, y, z): (usize, usize, usize)) -> Result<Self, Self::Error> {
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
            z: z.try_into()?,
        })
    }
}
