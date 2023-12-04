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
    fn points(&self) -> u32 {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
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

pub fn main(data: crate::DataIn) -> String {
    let mut ret = 0;
    for line in data {
        let card: Card = line.parse().unwrap();
        let points = card.points();
        println!("{card} | {points}");
        ret += points;
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "4",
    func: main,
});
