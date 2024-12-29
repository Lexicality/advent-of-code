// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{utils::bigcoord2d::BigCoord2D, AoCError, Coordinate2D, Direction};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for Colour {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(..)(..)(..)$").unwrap();
        }
        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Colour {s} does not match regex!")))?;
        Ok(Colour {
            red: u8::from_str_radix(&matches[1], 16).map_err(AoCError::new_from_parseerror)?,
            green: u8::from_str_radix(&matches[2], 16).map_err(AoCError::new_from_parseerror)?,
            blue: u8::from_str_radix(&matches[3], 16).map_err(AoCError::new_from_parseerror)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Instruction {
    dir: Direction,
    len: u64,
}

impl FromStr for Instruction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^. \d+ \(#(.+)(.)\)$").unwrap();
        }

        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Line {s} does not match regex!")))?;

        Ok(Instruction {
            dir: match &matches[2] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                v => v,
            }
            .parse()?,
            len: u64::from_str_radix(&matches[1], 16).map_err(AoCError::new_from_parseerror)?,
        })
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let instructions: Vec<Instruction> = data.map(|line| line.parse()).try_collect()?;
    let mut pos: BigCoord2D = Default::default();
    let n = instructions.len();

    let mut coords = Vec::with_capacity(n + 2);
    coords.push(pos);

    let mut exterior = 0;
    for instruction in instructions.into_iter() {
        let dir = instruction.dir.into();
        for _ in 0..instruction.len {
            pos += dir;
            exterior += 1;
        }
        coords.push(pos);
    }
    coords.push(coords[1]);

    let (xs, ys): (Vec<i64>, Vec<i64>) = coords.into_iter().map(|c| c.to_tuple()).unzip();

    let interior = 0.5
        * (1..=n)
            .map(|i| ys[i] * (xs[i - 1] - xs[i + 1]))
            .map(|i| i as f64)
            .sum::<f64>();

    Ok((interior as usize + (exterior / 2) + 1).to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "18",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
