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

use crate::{partition_input, AoCError};

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let (setup, puzzle) = partition_input(data);

    let dependencies = setup
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            (
                before
                    .parse()
                    .map_err(AoCError::new_from_parseerror)
                    .unwrap(),
                after
                    .parse()
                    .map_err(AoCError::new_from_parseerror)
                    .unwrap(),
            )
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<u32, HashSet<u32>>, (before, after): (u32, u32)| {
                acc.entry(after).or_default().insert(before);
                acc
            },
        );

    let ret: u32 = puzzle
        .map(|line| -> Vec<u32> {
            line.split(',')
                .map(str::parse)
                .try_collect()
                .map_err(AoCError::new_from_parseerror)
                .unwrap()
        })
        .filter(|pages| {
            for (i, page) in pages.iter().enumerate() {
                let Some(bads) = dependencies.get(page) else {
                    continue;
                };
                if pages.iter().skip(i + 1).any(|page| bads.contains(page)) {
                    return false;
                }
            }
            true
        })
        .map(|pages| pages[pages.len() / 2])
        .sum();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let (setup, puzzle) = partition_input(data);

    let dependencies = setup
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            (
                before
                    .parse()
                    .map_err(AoCError::new_from_parseerror)
                    .unwrap(),
                after
                    .parse()
                    .map_err(AoCError::new_from_parseerror)
                    .unwrap(),
            )
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<u32, HashSet<u32>>, (before, after): (u32, u32)| {
                acc.entry(after).or_default().insert(before);
                acc
            },
        );

    fn find_bads(
        pages: &[u32],
        dependencies: &HashMap<u32, HashSet<u32>>,
    ) -> Option<(usize, usize)> {
        for (i, page) in pages.iter().enumerate() {
            let Some(bads) = dependencies.get(page) else {
                continue;
            };
            if let Some((j, _)) = pages
                .iter()
                .enumerate()
                .skip(i + 1)
                .find(|(_, page)| bads.contains(page))
            {
                return Some((i, j));
            }
        }
        None
    }

    let ret: u32 = puzzle
        .map(|line| -> Vec<u32> {
            line.split(',')
                .map(str::parse)
                .try_collect()
                .map_err(AoCError::new_from_parseerror)
                .unwrap()
        })
        .filter(|pages| find_bads(pages, &dependencies).is_some())
        .map(|mut pages| {
            while let Some((i, j)) = find_bads(&pages, &dependencies) {
                pages.swap(i, j);
            }
            pages
        })
        .map(|pages| pages[pages.len() / 2])
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "5",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
