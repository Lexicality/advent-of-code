// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut destinations = HashSet::new();

    let routes = data
        .map(|line| {
            let (from, line) = line.split_once(" to ").unwrap();
            let (to, distance) = line.split_once(" = ").unwrap();
            destinations.insert(from.to_owned());
            destinations.insert(to.to_owned());
            (
                from.to_owned(),
                to.to_owned(),
                distance.parse::<u64>().unwrap(),
            )
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, HashMap<String, u64>>, (from, to, distance)| {
                acc.entry(from.clone())
                    .or_default()
                    .insert(to.clone(), distance);
                acc.entry(to).or_default().insert(from, distance);
                acc
            },
        );

    let ret: u64 = destinations
        .iter()
        .permutations(destinations.len())
        .map(|route| {
            route
                .into_iter()
                .tuple_windows()
                .map(|(from, to)| routes[from][to])
                .sum()
        })
        .max()
        .unwrap();

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "9",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
