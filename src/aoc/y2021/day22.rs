// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, AoCResult, CommonGrid, Coord3D, InfGrid};

#[derive(Debug, Clone, Copy)]
enum CubeState {
    Off,
    On,
}
impl CubeState {
    fn parse(input: &str) -> AoCResult<Self> {
        Ok(match input {
            "on" => Self::On,
            "off" => Self::Off,
            _ => return Err(AoCError::new(format!("Unknown cube state '{input}'"))),
        })
    }
}

#[derive(Debug, Clone)]
struct RebootStep {
    state: CubeState,
    min: Coord3D,
    max: Coord3D,
}

impl RebootStep {
    fn parse(line: String) -> AoCResult<Self> {
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$")
                    .unwrap();
        }

        let matches = LINE_RE
            .captures(&line)
            .ok_or_else(|| AoCError::new(format!("line {line} doesn't match regex")))?;
        Ok(Self {
            state: CubeState::parse(&matches[1])?,
            min: Coord3D {
                x: matches[2].parse().map_err(AoCError::new_from_parseerror)?,
                y: matches[4].parse().map_err(AoCError::new_from_parseerror)?,
                z: matches[6].parse().map_err(AoCError::new_from_parseerror)?,
            },
            max: Coord3D {
                x: matches[3].parse().map_err(AoCError::new_from_parseerror)?,
                y: matches[5].parse().map_err(AoCError::new_from_parseerror)?,
                z: matches[7].parse().map_err(AoCError::new_from_parseerror)?,
            },
        })
    }

    fn get_coords(&self) -> impl Iterator<Item = Coord3D> {
        (self.min.x..=self.max.x)
            .cartesian_product(self.min.y..=self.max.y)
            .cartesian_product(self.min.z..=self.max.z)
            .map(|((x, y), z)| Coord3D { x, y, z })
    }

    fn is_legit(&self) -> bool {
        // const MAGIC_SIZE: std::ops::RangeInclusive<i32> = -50..=50;

        (self.min.x <= 50 && self.max.x >= -50)
            && (self.min.y <= 50 && self.max.y >= -50)
            && (self.min.z <= 50 && self.max.z >= -50)
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid: InfGrid<CubeState, Coord3D> = InfGrid::new();

    for line in data {
        let step = RebootStep::parse(line)?;
        if !(step.is_legit()) {
            continue;
        }
        println!("step: {step:?}");
        for coord in step.get_coords()
        // .filter(|coord| {
        //     MAGIC_SIZE.contains(&coord.x)
        //         && MAGIC_SIZE.contains(&coord.y)
        //         && MAGIC_SIZE.contains(&coord.z)
        // })
        {
            grid.set(coord, step.state);
        }
    }

    let ret = grid
        .into_iter()
        .filter(|(_, v)| matches!(v, CubeState::On))
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2021",
    day: "22",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1
    }),
    part_2: None,
});
