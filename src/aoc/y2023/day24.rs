// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

use crate::{AoCError, AoCResult};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct CoordF3D {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for CoordF3D {
    type Err = AoCError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .map(|c| c.trim().parse().map_err(AoCError::new_from_parseerror))
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

impl std::ops::Mul<f64> for CoordF3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Display for CoordF3D {
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

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    start: CoordF3D,
    dir: CoordF3D,
}

impl FromStr for Hailstone {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, dir) = s.split_once('@').ok_or_else(|| {
            AoCError::new(format!("Expected {s} to have a '@' in it, but it didn't"))
        })?;

        Ok(Self {
            start: start.parse()?,
            dir: dir.parse()?,
        })
    }
}

fn stones(data: crate::DataIn) -> AoCResult<Vec<Hailstone>> {
    data.map(|line| line.parse()).try_collect()
}

fn collisions(hailstones: impl Iterator<Item = Hailstone>) -> impl Iterator<Item = CoordF3D> {
    // [m1.x, m1.y]*t + [b1.x, b1.y] = [m2.x, m2.y]*s + [b2.x, b2.y]
    // [s, t] = 1/(m1.x * -m2.y + m2.x * m1.y) * [(b1.x - b2.x), (b1.y, b2.y)]
    // a2 := Vector2{a.vel.x, a.vel.y}
    // b2 := Vector2{b.vel.x, b.vel.y}
    // d2 := Vector2{b.pos.x - a.pos.x, b.pos.y - a.pos.y}

    // det := vectorCross(a2, b2)
    // // parallel
    // if det == 0 {
    // 	return Vector2{-1, -1}, false
    // }

    // u := vectorCross(d2, b2) / det
    // return Vector2{a.pos.x + a.vel.x*u, a.pos.y + a.vel.y*u}, true

    //
    // func vectorCross(a, b Vector2) float64 {
    // 	return (a.x * b.y) - (a.y * b.x)
    // }

    hailstones.permutations(2).filter_map(|stones| {
        let (a, b) = stones.into_iter().collect_tuple().unwrap();

        let det = (a.dir.x * b.dir.y) - (a.dir.y * b.dir.x);
        if det == 0.0 {
            return None;
        }
        let wat = CoordF3D {
            x: b.start.x - a.start.x,
            y: b.start.y - a.start.y,
            z: 0.0,
        };

        let wat2 = (wat.x * a.start.y) - (wat.y * a.start.x);
        let wat2 = wat2 / det;

        Some(CoordF3D {
            x: a.start.x + a.dir.x * wat2,
            y: a.start.y + a.dir.y * wat2,
            z: 0.0,
        })
    })
}

#[allow(dead_code)]
pub fn part_1(_data: crate::DataIn) -> AoCResult<String> {
    // hailstones.iter().for_each(|stone| println!("{stone:?}"));
    let ret = 0;
    Ok(ret.to_string())
}

pub fn part_1_example(data: crate::DataIn) -> AoCResult<String> {
    let hailstones = stones(data)?;
    let collisions = collisions(hailstones.into_iter());

    Ok(collisions.inspect(|c| println!("{c}")).count().to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "24",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1_example
    }),
    part_2: None,
});
