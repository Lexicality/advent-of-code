use std::{fmt::Display, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, CommonGrid, Coordinate2D, Direction, InfGrid};

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
    len: u32,
    colour: Colour,
}

impl FromStr for Instruction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.) (\d+) \((#.+)\)$").unwrap();
        }

        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Line {s} does not match regex!")))?;

        Ok(Instruction {
            dir: matches[1].parse()?,
            len: matches[2].parse().map_err(AoCError::new_from_parseerror)?,
            colour: matches[3].parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
enum GridState {
    Ground,
    Edge(Colour),
    Dug,
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => '·',
            Self::Edge(_) => '█',
            Self::Dug => '▒',
        }
        .fmt(f)
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let instructions: Vec<Instruction> = data.map(|line| line.parse()).try_collect()?;
    let mut grid: InfGrid<GridState> = InfGrid::new();
    let mut pos = Default::default();
    grid.set(
        pos,
        GridState::Edge(Colour {
            red: 0,
            green: 0,
            blue: 0,
        }),
    );
    let n = instructions.len();
    let mut coords = Vec::with_capacity(n + 2);
    coords.push(pos);
    for instruction in instructions.into_iter() {
        let edgepiece = GridState::Edge(instruction.colour);
        let dir = instruction.dir.into();
        for _ in 0..instruction.len {
            pos += dir;
            grid.set(pos, edgepiece);
        }
        coords.push(pos);
    }
    coords.push(coords[1]);

    let (xs, ys): (Vec<i32>, Vec<i32>) = coords.into_iter().map(|c| c.to_tuple()).unzip();

    let interior = 0.5
        * (1..=n)
            .map(|i| ys[i] * (xs[i - 1] - xs[i + 1]))
            .map(|i| i as f64)
            .sum::<f64>();

    let exterior = grid.into_iter().count();

    Ok((interior as usize + (exterior / 2) + 1).to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "18",
    func: main,
    example_func: None,
});
