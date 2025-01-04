// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{collections::HashMap, str::FromStr};

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

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let towels: Vec<Towel> = data
        .next()
        .unwrap()
        .split(", ")
        .map(str::parse)
        .try_collect()?;

    assert!(data.next().unwrap().is_empty());

    let ret = data
        .filter(|line| {
            let mut current_options: Vec<&[char]> =
                towels.iter().map(|towel| &towel.letters[..]).collect();
            for char in line.chars() {
                let mut add_new = false;
                current_options = current_options
                    .into_iter()
                    .filter_map(|stripes| match stripes.split_first() {
                        Some((c, ret)) if *c == char => Some(ret),
                        Some(_) => None,
                        None => {
                            add_new = true;
                            None
                        }
                    })
                    .collect();
                if add_new {
                    current_options.extend(
                        towels
                            .iter()
                            .filter(|towel| towel.starts_with == char)
                            .map(|towel| &towel.letters[1..]),
                    );
                }
                if current_options.is_empty() {
                    return false;
                }
            }
            current_options
                .into_iter()
                .any(|stripes| stripes.is_empty())
        })
        .count();

    // let ret = data.count();
    Ok(ret.to_string())
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let towels: Vec<Towel> = data
        .next()
        .unwrap()
        .split(", ")
        .map(str::parse)
        .try_collect()?;

    assert!(data.next().unwrap().is_empty());

    let ret: usize = data
        .filter_map(|line| {
            let mut current_options: HashMap<&[char], usize> =
                towels.iter().map(|towel| (&towel.letters[..], 1)).collect();
            for char in line.chars() {
                current_options = current_options
                    .into_iter()
                    .flat_map(|(stripes, counts)| match stripes.split_first() {
                        Some((c, ret)) if *c == char => vec![(ret, counts)],
                        Some(_) => vec![],
                        None => towels
                            .iter()
                            .filter(|towel| towel.starts_with == char)
                            .map(|towel| (&towel.letters[1..], counts))
                            .collect(),
                    })
                    .fold(
                        HashMap::with_capacity(towels.len()),
                        |mut acc, (stripes, count)| {
                            *acc.entry(stripes).or_default() += count;
                            acc
                        },
                    );
                if current_options.is_empty() {
                    return None;
                }
            }
            Some(
                current_options
                    .into_iter()
                    .filter(|(stripes, _)| stripes.is_empty())
                    .map(|(_, count)| count)
                    .sum::<usize>(),
            )
        })
        .sum();

    // let ret = data.count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "19",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
