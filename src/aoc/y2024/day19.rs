// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::str::FromStr;

use itertools::Itertools;

use crate::AoCError;

struct Towel {
    letters: Vec<char>,
    starts_with: char,
}

impl FromStr for Towel {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            letters: s.chars().collect(),
            starts_with: s.chars().next().ok_or(AoCError::new("Towel is empty?!"))?,
        })
    }
}

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let towels: Vec<Towel> = data
        .next()
        .unwrap()
        .split(", ")
        .map(str::parse)
        .try_collect()?;

    assert!(data.next().unwrap().is_empty());

    let ret: usize = data
        .filter_map(|line| {
            let mut current_options: Vec<&[char]> =
                towels.iter().map(|towel| &towel.letters[..]).collect();
            for char in line.chars() {
                current_options = current_options
                    .into_iter()
                    .flat_map(|stripes| match stripes.split_first() {
                        Some((c, ret)) if *c == char => vec![ret],
                        Some(_) => vec![],
                        None => towels
                            .iter()
                            .filter(|towel| towel.starts_with == char)
                            .map(|towel| &towel.letters[1..])
                            .collect(),
                    })
                    .collect();
                if current_options.is_empty() {
                    return None;
                }
            }
            Some(
                current_options
                    .into_iter()
                    .filter(|stripes| stripes.is_empty())
                    .count(),
            )
        })
        .sum();

    // let ret = data.count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "19", main));
