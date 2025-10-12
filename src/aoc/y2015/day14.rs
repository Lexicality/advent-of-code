// Copyright (c) 2025 Lexi Robinson
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
use num::Integer;
use regex::Regex;

use crate::AoCError;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

impl Reindeer {
    fn cycle_time(&self) -> usize {
        self.fly_time + self.rest_time
    }

    fn cycle_distance(&self) -> usize {
        self.speed * self.fly_time
    }

    fn get_distance(&self, time: usize) -> usize {
        let cycle_time = self.cycle_time();
        let (complete_cycles, remainder) = time.div_rem(&cycle_time);

        let cycle_distance = self.cycle_distance();

        complete_cycles * cycle_distance + {
            if remainder >= self.fly_time {
                cycle_distance
            } else {
                remainder * self.speed
            }
        }
    }
}

impl FromStr for Reindeer {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^([^ ]+) can fly (\d+) km/s for (\d+) .+ (\d+) seconds.$").unwrap();
        }

        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Line {s} does not match regex")))?;

        Ok(Self {
            name: matches[1].to_owned(),
            speed: matches[2].parse().map_err(AoCError::new_from_parseerror)?,
            fly_time: matches[3].parse().map_err(AoCError::new_from_parseerror)?,
            rest_time: matches[4].parse().map_err(AoCError::new_from_parseerror)?,
        })
    }
}

pub fn part_1(data: crate::DataIn, time: usize) -> crate::AoCResult<String> {
    let reindeer: Vec<Reindeer> = data.map(|line| line.parse()).try_collect()?;

    for reindeer in reindeer.iter() {
        println!("{} flew {} km", reindeer.name, reindeer.get_distance(time));
    }

    reindeer
        .into_iter()
        .map(|reindeer| reindeer.get_distance(time))
        .max()
        .map(|v| v.to_string())
        .ok_or(AoCError::new("No reindeer??"))
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "14",
    part_1: crate::AoCPart {
        main: |data| part_1(data, 2503),
        example: |data| part_1(data, 1000),
    },
    part_2: None
});
