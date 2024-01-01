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
    fn act(&self, value: bool) -> bool {
        match self {
            Self::TurnOn => true,
            Self::TurnOff => false,
            Self::Toggle => !value,
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

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::<bool>::new_filled(1000, 1000, false);
    for command in data.map(|line| line.parse()) {
        let command: Command = command?;
        for x in command.start.x..=command.end.x {
            for y in command.start.y..=command.end.y {
                let value = grid.get_mut(&(x, y).into()).unwrap();
                *value = command.action.act(*value);
            }
        }
    }
    Ok(grid.into_iter().filter(|(_, v)| *v).count().to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "6", main));
