// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, AoCResult};

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl Card {
    fn num_winning_nums(&self) -> usize {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .count()
    }
}

fn invalid_number(cause: ParseIntError) -> AoCError {
    AoCError::new_with_cause("Invalid number", cause)
}

fn parse_numbers(nums: &str) -> AoCResult<HashSet<u32>> {
    nums.split(' ')
        .filter(|c| !c.is_empty())
        .map(|num| num.parse().map_err(invalid_number))
        .collect()
}

impl FromStr for Card {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Card\s+(\d+): (.+) \| (.+)$").unwrap();
        }
        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Line {s} does not match regex!")))?;

        Ok(Card {
            id: matches[1].parse().map_err(invalid_number)?,
            winning_numbers: parse_numbers(&matches[2])?,
            card_numbers: parse_numbers(&matches[3])?,
        })
    }
}

fn fmt_nmubers(f: &mut std::fmt::Formatter<'_>, nums: &HashSet<u32>) -> std::fmt::Result {
    write!(
        f,
        "{}",
        nums.iter().map(|num| format!("{num:>2}")).join(" ")
    )
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {:>3}: ", self.id)?;
        fmt_nmubers(f, &self.winning_numbers)?;
        write!(f, " | ")?;
        fmt_nmubers(f, &self.card_numbers)
    }
}

#[derive(Debug)]
struct CardQueue(VecDeque<u64>);

impl CardQueue {
    fn pop(&mut self) -> u64 {
        self.0.pop_front().unwrap_or(0)
    }

    fn incr(&mut self, amt: usize) {
        for index in 0..amt {
            if let Some(count) = self.0.get_mut(index) {
                *count += 1;
            } else {
                self.0.push_back(1);
            }
        }
    }

    fn new() -> Self {
        CardQueue(VecDeque::new())
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret: u64 = 0;
    let mut queue = CardQueue::new();
    for line in data {
        let card: Card = line.parse().unwrap();
        let points = card.num_winning_nums();
        let count = queue.pop() + 1;
        if points > 0 {
            for _ in 0..count {
                queue.incr(points);
            }
        }
        ret += count;
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2023", "4", main));
