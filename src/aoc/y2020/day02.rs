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
use regex::Regex;

struct Policy {
    a: usize,
    b: usize,
    letter: char,
    password: String,
}

impl FromStr for Policy {
    type Err = crate::AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+).(\d+) (.): (.+)$").unwrap();
        }

        let matches = RE
            .captures(s)
            .ok_or_else(Self::Err::new_from_regex(s, &RE))?;

        Ok(Self {
            a: matches[1].parse()?,
            b: matches[2].parse()?,
            letter: matches[3].chars().next().unwrap(),
            password: matches[4].to_owned(),
        })
    }
}

impl Policy {
    fn part_1(&self) -> bool {
        let range = self.a..=self.b;
        range.contains(&(self.password.chars().filter(|c| *c == self.letter).count()))
    }

    fn part_2(&self) -> bool {
        let chars = self.password.chars().collect_vec();
        chars.get(self.a - 1).is_some_and(|c| *c == self.letter)
            ^ chars.get(self.b - 1).is_some_and(|c| *c == self.letter)
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let policies: Vec<Policy> = data.map(|line| line.parse()).try_collect()?;
    let ret = policies.into_iter().filter(|p| p.part_1()).count();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let policies: Vec<Policy> = data.map(|line| line.parse()).try_collect()?;
    let ret = policies.into_iter().filter(|p| p.part_2()).count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2020",
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
