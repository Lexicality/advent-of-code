// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret: usize = data
        .batching(|iter| {
            let set: HashSet<char> = iter
                .take_while(|line| !line.is_empty())
                .flat_map(|line| line.chars().collect_vec())
                .collect();

            if set.is_empty() {
                None
            } else {
                Some(set.len())
            }
        })
        .sum();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret: usize = data
        .batching(|iter| {
            let my_lines = iter.take_while(|line| !line.is_empty()).collect_vec();

            if my_lines.is_empty() {
                return None;
            }

            let target_count = my_lines.len();

            let mut charcounts: HashMap<char, usize> = HashMap::new();

            my_lines.into_iter().for_each(|line| {
                line.chars().for_each(|c| {
                    charcounts.entry(c).and_modify(|i| *i += 1).or_insert(1);
                });
            });

            Some(charcounts.values().filter(|i| **i >= target_count).count())
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2020",
    day: "6",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
