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

use crate::{AoCError, partition_input};

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let (setup, puzzle) = partition_input(data);

    let dependencies = setup
        .map(|line| -> Result<_, AoCError> {
            let (before, after) = line.split_once('|').unwrap();
            Ok((before.parse()?, after.parse()?))
        })
        .fold_ok(
            HashMap::new(),
            |mut acc: HashMap<u32, HashSet<u32>>, (before, after): (u32, u32)| {
                acc.entry(after).or_default().insert(before);
                acc
            },
        )?;

    let ret: u32 = puzzle
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .try_collect::<_, Vec<u32>, _>()
                .map_err(AoCError::new_from_parseerror)
        })
        .filter_ok(|pages| {
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
        .map_ok(|pages| pages[pages.len() / 2])
        .fold_ok(0, std::ops::Add::add)?;
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let (setup, puzzle) = partition_input(data);

    let dependencies = setup
        .map(|line| -> Result<_, AoCError> {
            let (before, after) = line.split_once('|').unwrap();
            Ok((before.parse()?, after.parse()?))
        })
        .fold_ok(
            HashMap::new(),
            |mut acc: HashMap<u32, HashSet<u32>>, (before, after): (u32, u32)| {
                acc.entry(after).or_default().insert(before);
                acc
            },
        )?;

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
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .try_collect::<_, Vec<u32>, _>()
                .map_err(AoCError::new_from_parseerror)
        })
        .filter_ok(|pages| find_bads(pages, &dependencies).is_some())
        .map_ok(|mut pages| {
            while let Some((i, j)) = find_bads(&pages, &dependencies) {
                pages.swap(i, j);
            }
            pages
        })
        .map_ok(|pages| pages[pages.len() / 2])
        .fold_ok(0, std::ops::Add::add)?;
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
