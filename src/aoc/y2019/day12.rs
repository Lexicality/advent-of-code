// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{cmp::Ordering, fmt::Display, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, Coord3D};

struct Moon {
    pos: Coord3D,
    vel: Coord3D,
}

fn energy(coord: Coord3D) -> u64 {
    let Coord3D { x, y, z } = coord;
    x.unsigned_abs() as u64 + y.unsigned_abs() as u64 + z.unsigned_abs() as u64
}

impl Moon {
    fn interact(&mut self, other: &mut Self) {
        match self.pos.x.cmp(&other.pos.x) {
            Ordering::Equal => (),
            Ordering::Greater => {
                self.vel.x -= 1;
                other.vel.x += 1;
            }
            Ordering::Less => {
                self.vel.x += 1;
                other.vel.x -= 1;
            }
        }
        match self.pos.y.cmp(&other.pos.y) {
            Ordering::Equal => (),
            Ordering::Greater => {
                self.vel.y -= 1;
                other.vel.y += 1;
            }
            Ordering::Less => {
                self.vel.y += 1;
                other.vel.y -= 1;
            }
        }
        match self.pos.z.cmp(&other.pos.z) {
            Ordering::Equal => (),
            Ordering::Greater => {
                self.vel.z -= 1;
                other.vel.z += 1;
            }
            Ordering::Less => {
                self.vel.z += 1;
                other.vel.z -= 1;
            }
        }
    }

    fn complete_step(&mut self) {
        self.pos += self.vel;
    }

    fn energy(&self) -> u64 {
        energy(self.pos) * energy(self.vel)
    }
}

impl FromStr for Moon {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
        }
        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Line {s} does not match regex!")))?;

        Ok(Moon {
            pos: Coord3D {
                x: matches[1].parse().map_err(AoCError::new_from_parseerror)?,
                y: matches[2].parse().map_err(AoCError::new_from_parseerror)?,
                z: matches[3].parse().map_err(AoCError::new_from_parseerror)?,
            },
            vel: Coord3D { x: 0, y: 0, z: 0 },
        })
    }
}

impl Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>",
            self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z,
        )
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let moons: Vec<Moon> = data.map(|line| line.parse()).try_collect().unwrap();

    let (mut io, mut europa, mut ganymede, mut callisto) =
        moons.into_iter().collect_tuple().unwrap();

    // println!("after 0 steps:\n{io}\n{europa}\n{ganymede}\n{callisto}\n");
    for _ in 1..=1000 {
        io.interact(&mut europa);
        io.interact(&mut ganymede);
        io.interact(&mut callisto);
        europa.interact(&mut ganymede);
        europa.interact(&mut callisto);
        ganymede.interact(&mut callisto);

        io.complete_step();
        europa.complete_step();
        ganymede.complete_step();
        callisto.complete_step();

        // println!("after {step} steps:\n{io}\n{europa}\n{ganymede}\n{callisto}\n");
    }

    Ok((io.energy() + europa.energy() + ganymede.energy() + callisto.energy()).to_string())
}

inventory::submit!(crate::AoCDay::mew("2019", "12", main));
