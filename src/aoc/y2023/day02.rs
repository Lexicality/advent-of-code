// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{fmt::Display, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;

use crate::AoCError;

#[derive(Debug)]
struct State {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Show {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Show {
    type Err = AoCError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Show {
            red: 0,
            green: 0,
            blue: 0,
        };
        for a in s.split(',').map(|s| {
            s.trim()
                .split_once(' ')
                .ok_or(AoCError::new("Shows must be two parts"))
        }) {
            let (num, colour) = a?;
            let num: u32 = num
                .parse()
                .map_err(|e| AoCError::new_with_cause("Shows must start with a number", e))?;
            match colour {
                "red" => ret.red += num,
                "blue" => ret.blue += num,
                "green" => ret.green += num,
                _ => return Err(AoCError::new("Shows must be red, blue or green")),
            };
        }
        Ok(ret)
    }
}

impl Display for Show {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            // CBA to do anything smart about hiding 0s
            "{} red, {} green, {} blue",
            self.red, self.green, self.blue,
        )
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    min_red: u32,
    min_green: u32,
    min_blue: u32,
    shows: Vec<Show>,
}

impl FromStr for Game {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Game (\d+): (.+)$").unwrap();
        }
        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Line {s} does not match regex!")))?;
        let id = &matches[1];
        let shows = &matches[2];
        let mut game = Game {
            id: id
                .parse()
                .map_err(|e| AoCError::new_with_cause("Game ID isn't a number", e))?,
            min_red: 0,
            min_green: 0,
            min_blue: 0,
            shows: Vec::with_capacity(shows.chars().filter(|c| c == &';').count()),
        };
        for line in shows.split(';') {
            let show: Show = line.parse()?;
            game.min_red = game.min_red.max(show.red);
            game.min_green = game.min_green.max(show.green);
            game.min_blue = game.min_blue.max(show.blue);
            game.shows.push(show);
        }

        Ok(game)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game {}: ", self.id)?;
        for show in self.shows.iter() {
            write!(f, "{}; ", show)?;
        }
        Ok(())
    }
}

impl Game {
    fn is_possible(&self, state: &State) -> bool {
        self.min_red > state.red || self.min_green > state.green || self.min_blue > state.blue
    }

    fn power(&self) -> u32 {
        self.min_red * self.min_green * self.min_blue
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    let state = State {
        red: 12,
        green: 13,
        blue: 14,
    };
    for line in data {
        let game: Game = line.parse().unwrap();
        println!("{game}");
        if !game.is_possible(&state) {
            println!("Impossible!");
            ret += game.id;
        }
    }
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    for line in data {
        let game: Game = line.parse().unwrap();
        println!("{game}");
        ret += game.power();
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "2",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
