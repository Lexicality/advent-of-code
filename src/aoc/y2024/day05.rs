use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{partition_input, AoCError};

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
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

inventory::submit!(crate::AoCDay::mew("2024", "5", main));
