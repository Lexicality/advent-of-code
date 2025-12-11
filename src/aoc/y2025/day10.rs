// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.
//

use std::cmp::Ordering;
use std::fmt::Display;
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::AoCError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct IndicatorState([bool; 10]);

impl FromStr for IndicatorState {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tmp: Vec<bool> = s
            .chars()
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(AoCError::new_from_char(c)),
            })
            .try_collect()?;
        match tmp.len().cmp(&10) {
            Ordering::Less => {
                tmp.extend((0..(10 - tmp.len())).map(|_| false));
            }
            Ordering::Equal => (),
            Ordering::Greater => {
                return Err(AoCError::new(format!("Input string {s} is too long!")));
            }
        }
        Ok(Self(tmp.into_iter().collect_array().unwrap()))
    }
}

impl Display for IndicatorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|state| if *state { '#' } else { '.' })
                .join("")
        )
    }
}

impl IndicatorState {
    fn be_pushed(mut self, button: &Button) -> Self {
        for btn in button.0.iter().copied() {
            self.0[btn] = !self.0[btn];
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct JoltState([u16; 10]);

impl FromStr for JoltState {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tmp: Vec<u16> = s.split(',').map(|s| s.parse()).try_collect()?;
        match tmp.len().cmp(&10) {
            Ordering::Less => {
                tmp.extend((0..(10 - tmp.len())).map(|_| 0));
            }
            Ordering::Equal => (),
            Ordering::Greater => {
                return Err(AoCError::new(format!("Input string {s} is too long!")));
            }
        }
        Ok(Self(tmp.into_iter().collect_array().unwrap()))
    }
}

impl Display for JoltState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.0.iter().join(","))
    }
}

impl JoltState {
    fn be_pushed(mut self, button: &Button) -> Self {
        for btn in button.0.iter().copied() {
            self.0[btn] += 1;
        }
        self
    }

    fn still_valid(&self, target: &Self) -> bool {
        std::iter::zip(self.0.iter(), target.0.iter()).all(|(me, them)| me <= them)
    }
}

#[derive(Debug)]
struct Button(Vec<usize>);

impl FromStr for Button {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim_start_matches('(')
                .trim_end_matches(')')
                .split(",")
                .map(|s| s.parse())
                .try_collect()?,
        ))
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0.iter().join(","))
    }
}

#[derive(Debug)]
struct Machine {
    target: IndicatorState,
    jolt_target: JoltState,
    buttons: Vec<Button>,
}

impl FromStr for Machine {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // I swear he intentionally makes these annoying to parse with regex
            static ref RE: Regex = Regex::new(r"^\[([#.]+)\] ((?:\([\d,]+\) ?)+) \{([\d,]+)\}$").unwrap();
        }

        let matches = RE
            .captures(s)
            .ok_or_else(Self::Err::new_from_regex(s, &RE))?;

        Ok(Self {
            target: matches[1].parse()?,
            jolt_target: matches[3].parse()?,
            buttons: matches[2]
                .split_whitespace()
                .map(|s| s.parse())
                .try_collect()?,
        })
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.target,
            self.buttons.iter().join(" "),
            self.jolt_target
        )
    }
}

impl Machine {
    fn part_1(&self) -> usize {
        struct PushResult {
            last_button: usize,
            state: IndicatorState,
        }

        let current_state: IndicatorState = Default::default();

        let mut pushes: Vec<PushResult> = self
            .buttons
            .iter()
            .enumerate()
            .map(|(i, btn)| PushResult {
                last_button: i,
                state: current_state.be_pushed(btn),
            })
            .collect();

        for i in 1.. {
            if pushes
                .iter()
                .any(|PushResult { state, .. }| state == &self.target)
            {
                return i;
            }

            // Let's get fucking stupid with it
            pushes = pushes
                .into_iter()
                .flat_map(|PushResult { last_button, state }| {
                    self.buttons
                        .iter()
                        .enumerate()
                        .filter(move |(i, _)| *i != last_button)
                        .map(move |(last_button, button)| PushResult {
                            last_button,
                            state: state.be_pushed(button),
                        })
                })
                .collect();
        }
        unreachable!();
    }

    fn part_2(&self) -> usize {
        struct PushResult {
            last_button: usize,
            state: JoltState,
        }

        let current_state: JoltState = Default::default();

        let mut pushes: Vec<PushResult> = self
            .buttons
            .iter()
            .enumerate()
            .map(|(i, btn)| PushResult {
                last_button: i,
                state: current_state.be_pushed(btn),
            })
            .collect();

        for i in 1.. {
            if pushes
                .iter()
                .any(|PushResult { state, .. }| state == &self.jolt_target)
            {
                return i;
            }

            assert!(!pushes.is_empty());

            // Let's get fucking stupid with it
            pushes = pushes
                .into_iter()
                .flat_map(|PushResult { last_button, state }| {
                    self.buttons
                        .iter()
                        .enumerate()
                        .filter(move |(i, _)| *i != last_button)
                        .map(move |(last_button, button)| PushResult {
                            last_button,
                            state: state.be_pushed(button),
                        })
                        .filter(|push| push.state.still_valid(&self.jolt_target))
                })
                .collect();
        }
        unreachable!();
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let machines: Vec<Machine> = data.parse().try_collect()?;
    let ret: usize = machines
        .into_iter()
        .map(|machine| {
            log::debug!("{machine}");
            machine.part_1()
        })
        .sum();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let machines: Vec<Machine> = data.parse().try_collect()?;
    let ret: usize = machines.into_iter().map(|machine| machine.part_2()).sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "10",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});
