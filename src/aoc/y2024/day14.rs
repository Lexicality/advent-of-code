// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{cmp::Ordering, collections::HashSet, fmt::Display, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{symbols, AoCError, AoCResult, CommonGrid, Coord2D, Grid};

const ITERATIONS: usize = 100;
// these are exclusive
const MAX_X: i32 = 101;
const MAX_Y: i32 = 103;
const MAX_X_EXAMPLE: i32 = 11;
const MAX_Y_EXAMPLE: i32 = 7;

struct Robot {
    pos: Coord2D,
    vel: Coord2D,
}

impl FromStr for Robot {
    type Err = AoCError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^p=(\d+,\d+) v=(-?\d+,-?\d+)$").unwrap();
        }
        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("line {s} does not match regex!")))?;

        Ok(Self {
            pos: matches[1].parse()?,
            vel: matches[2].parse()?,
        })
    }
}

impl Robot {
    fn simulate(&mut self, max_x: i32, max_y: i32) {
        // print!("I'm at {} and I'm moving {} ", self.pos, self.vel);
        self.pos += self.vel;
        // print!("to {}", self.pos);
        if self.pos.x >= max_x {
            self.pos.x -= max_x;
            // print!(" x too big!!");
        } else if self.pos.x < 0 {
            self.pos.x += max_x;
            // print!(" x too smol!!");
        }
        if self.pos.y >= max_y {
            self.pos.y -= max_y;
            // print!(" y too big!!");
        } else if self.pos.y < 0 {
            self.pos.y += max_y;
            // print!(" y too smol!!");
        }
        // println!();
    }
}

#[derive(Debug, Default)]
struct RobotCount(usize);

impl Display for RobotCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => symbols::VOID.fmt(f),
            i if i < 10 => i.fmt(f),
            _ => symbols::INVALID.fmt(f),
        }
    }
}

fn debug_grid(robots: &[Robot], max_x: i32, max_y: i32) {
    let mut debuggrid: Grid<RobotCount> = Grid::new(
        //
        u32::try_from(max_x).unwrap(),
        u32::try_from(max_y).unwrap(),
    );
    for robot in robots.iter() {
        debuggrid.get_mut(&robot.pos).unwrap().0 += 1;
    }
    println!("{debuggrid:#}");
}

fn part_1(data: crate::DataIn, max_x: i32, max_y: i32) -> AoCResult<String> {
    // L + gridless
    let mut robots: Vec<Robot> = data.map(|line| line.parse()).try_collect()?;
    // println!("Initial State");
    // debug_grid(&robots, max_x, max_y);
    for _i in 0..ITERATIONS {
        for robot in robots.iter_mut() {
            robot.simulate(max_x, max_y);
        }
        // println!("After {_i} seconds");
        // debug_grid(&robots, max_x, max_y);
    }
    debug_grid(&robots, max_x, max_y);
    // quadrentify
    let mid_x = max_x / 2;
    let mid_y = max_y / 2;
    let quads = robots.into_iter().fold([0; 4], |mut acc, robot| {
        let pos = robot.pos;
        match (pos.x.cmp(&mid_x), pos.y.cmp(&mid_y)) {
            (Ordering::Less, Ordering::Less) => acc[0] += 1,
            (Ordering::Less, Ordering::Greater) => acc[1] += 1,
            (Ordering::Greater, Ordering::Less) => acc[2] += 1,
            (Ordering::Greater, Ordering::Greater) => acc[3] += 1,
            _ => (),
        }
        acc
    });
    let ret: u32 = quads.into_iter().product();
    Ok(ret.to_string())
}

fn paws() -> AoCResult<bool> {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|cause| AoCError::new_with_cause("Failed to read from stdin", cause))?;
    Ok(buffer.trim().to_lowercase() == "y")
}

fn robots_unique(robots: &[Robot]) -> bool {
    let mut seen = HashSet::with_capacity(robots.len());
    robots.iter().all(|robot| seen.insert(robot.pos))
}

fn part_2(data: crate::DataIn) -> AoCResult<String> {
    let mut robots: Vec<Robot> = data.map(|line| line.parse()).try_collect()?;
    for i in 1.. {
        for robot in robots.iter_mut() {
            robot.simulate(MAX_X, MAX_Y);
        }
        if robots_unique(&robots) {
            debug_grid(&robots, MAX_X, MAX_Y);
            println!("Correct (@ {i})?");
            if paws()? {
                return Ok(i.to_string());
            }
        }
    }
    unreachable!()
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "14",
    part_1: crate::AoCPart {
        main: |data| part_1(data, MAX_X, MAX_Y),
        example: |data| part_1(data, MAX_X_EXAMPLE, MAX_Y_EXAMPLE)
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |_| Err(AoCError::new("No example for part 2!"))
    }),
});
