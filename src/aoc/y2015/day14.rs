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
enum State {
    Flap(usize),
    Eep(usize),
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
    distance: usize,
    points: usize,
    state: State,
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

    fn tick(&mut self) {
        match self.state {
            State::Flap(time_left) => {
                self.distance += self.speed;
                let time_left = time_left - 1;
                if time_left > 0 {
                    self.state = State::Flap(time_left)
                } else {
                    self.state = State::Eep(self.rest_time)
                }
            }
            State::Eep(time_left) => {
                let time_left = time_left - 1;
                if time_left > 0 {
                    self.state = State::Eep(time_left)
                } else {
                    self.state = State::Flap(self.fly_time)
                }
            }
        }
    }
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Reindeer {}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
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

        let fly_time = matches[3].parse().map_err(AoCError::new_from_parseerror)?;
        Ok(Self {
            name: matches[1].to_owned(),
            speed: matches[2].parse().map_err(AoCError::new_from_parseerror)?,
            fly_time,
            rest_time: matches[4].parse().map_err(AoCError::new_from_parseerror)?,
            distance: 0,
            points: 0,
            state: State::Flap(fly_time),
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

pub fn part_2(data: crate::DataIn, time: usize) -> crate::AoCResult<String> {
    let mut reindeer: Vec<Reindeer> = data.map(|line| line.parse()).try_collect()?;

    // so much for having done it the smart way the first time
    for _ in 0..time {
        reindeer.iter_mut().for_each(Reindeer::tick);
        // Can't think of a less stupid way rn
        let max_dist = reindeer.iter_mut().max().unwrap().distance;
        reindeer
            .iter_mut()
            .filter(|r| r.distance == max_dist)
            .for_each(|r| r.points += 1);
    }

    for reindeer in reindeer.iter() {
        println!(
            "{} flew {} km and scored {} points",
            reindeer.name, reindeer.distance, reindeer.points
        );
    }

    Ok(reindeer
        .into_iter()
        .map(|r| r.points)
        .max()
        .unwrap()
        .to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "14",
    part_1: crate::AoCPart {
        main: |data| part_1(data, 2503),
        example: |data| part_1(data, 1000),
    },
    part_2: Some(crate::AoCPart {
        main: |data| part_2(data, 2503),
        example: |data| part_2(data, 1000),
    }),
});
