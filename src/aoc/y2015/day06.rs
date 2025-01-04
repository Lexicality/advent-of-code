// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, CommonGrid, Coord2D, Grid};

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Action {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => Ok(Self::TurnOn),
            "turn off" => Ok(Self::TurnOff),
            "toggle" => Ok(Self::Toggle),
            _ => Err(AoCError::new(format!("Unknown action {s}"))),
        }
    }
}

impl Action {
    fn act_part_1(&self, value: bool) -> bool {
        match self {
            Self::TurnOn => true,
            Self::TurnOff => false,
            Self::Toggle => !value,
        }
    }
    fn act_part_2(&self, value: u32) -> u32 {
        match self {
            Self::TurnOn => value.checked_add(1).expect("u32 overflow!!"),
            Self::TurnOff => value.saturating_sub(1),
            Self::Toggle => value.checked_add(2).expect("u32 overflow!!"),
        }
    }
}

struct Command {
    action: Action,
    start: Coord2D,
    end: Coord2D,
}

impl FromStr for Command {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+) (\d+,\d+) through (\d+,\d+)$").unwrap();
        }
        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Command {s} does not match regex!")))?;
        Ok(Self {
            action: matches[1].parse()?,
            start: matches[2].parse()?,
            end: matches[3].parse()?,
        })
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::<bool>::new_filled(1000, 1000, false);
    for command in data.map(|line| line.parse()) {
        let command: Command = command?;
        for x in command.start.x..=command.end.x {
            for y in command.start.y..=command.end.y {
                let value = grid.get_mut(&(x, y).into()).unwrap();
                *value = command.action.act_part_1(*value);
            }
        }
    }
    Ok(grid.into_iter().filter(|(_, v)| *v).count().to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::<u32>::new_filled(1000, 1000, 0);
    for command in data.map(|line| line.parse()) {
        let command: Command = command?;
        for x in command.start.x..=command.end.x {
            for y in command.start.y..=command.end.y {
                let value = grid.get_mut(&(x, y).into()).unwrap();
                *value = command.action.act_part_2(*value);
            }
        }
    }
    Ok(grid
        .into_iter()
        .map(|(_, v)| v as u64)
        .sum::<u64>()
        .to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "6",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
