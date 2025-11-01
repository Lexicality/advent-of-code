// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::Direction;

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut instructions = data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::East,
            'R' => Direction::West,
            _ => unreachable!(),
        })
        .collect_vec()
        .into_iter()
        .cycle();

    assert_eq!(data.next(), Some("".to_owned()));

    let nodes: HashMap<String, (String, String)> = data
        .map(|line| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(...) = \((...), (...)\)$").unwrap();
            }
            let matches = RE
                .captures(&line)
                .unwrap_or_else(|| panic!("{line} didn't match regex"));

            (
                matches[1].to_owned(),
                (matches[2].to_owned(), matches[3].to_owned()),
            )
        })
        .collect();

    // println!("{nodes:#?}");

    let mut ret = 0;
    let mut pos = "AAA";
    while pos != "ZZZ" {
        ret += 1;
        let node = nodes
            .get(pos)
            .unwrap_or_else(|| panic!("don't have a node for {pos}!!!"));
        // print!("{pos}, {node:?} ");
        pos = match instructions.next().unwrap() {
            Direction::East => &node.0,
            Direction::West => &node.1,
            _ => unreachable!(),
        };
        // println!("=> {pos}");
    }
    Ok(ret.to_string())
}

fn navigate<'a>(
    instruction: Direction,
    nodes: &'a HashMap<String, (String, String)>,
    pos: &'a str,
) -> &'a str {
    let node = nodes
        .get(pos)
        .unwrap_or_else(|| panic!("don't have a node for {pos}!!!"));
    match instruction {
        Direction::East => &node.0,
        Direction::West => &node.1,
        _ => unreachable!(),
    }
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let raw_instructions = data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::East,
            'R' => Direction::West,
            _ => unreachable!(),
        })
        .collect_vec();

    let num_instructions = raw_instructions.len();

    let mut instructions = raw_instructions.into_iter().cycle();

    assert_eq!(data.next(), Some("".to_owned()));

    let nodes: HashMap<String, (String, String)> = data
        .map(|line| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(...) = \((...), (...)\)$").unwrap();
            }
            let matches = RE
                .captures(&line)
                .unwrap_or_else(|| panic!("{line} didn't match regex"));

            (
                matches[1].to_owned(),
                (matches[2].to_owned(), matches[3].to_owned()),
            )
        })
        .collect();

    let mut ret = num_instructions;

    let poses = nodes
        .keys()
        .filter(|name| name.ends_with('A'))
        .map(|name| name.as_str())
        .collect_vec();
    for mut pos in poses {
        let mut num_steps: usize = 0;
        while !pos.ends_with('Z') {
            num_steps += 1;
            let instruction = instructions.next().unwrap();
            pos = navigate(instruction, &nodes, pos);
        }
        let num_cycles = num_steps / num_instructions;
        ret *= num_cycles;
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "8",
    part_1: crate::AoCPart {
        main: part_1,
        // example modified for part 2, no longer compatible
        example: crate::no_example
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
