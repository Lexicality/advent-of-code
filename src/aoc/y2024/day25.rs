// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::{Either, Itertools};

use crate::{AoCError, AoCResult, InputPartitioner};

enum Pinnable {
    Lock([u8; 5]),
    Key([u8; 5]),
}

impl Pinnable {
    fn new(lines: Vec<String>) -> AoCResult<Self> {
        let mut lines = lines.into_iter();
        let frist = lines
            .next()
            .ok_or(AoCError::new("must have at least one line"))?;
        let last = lines
            .next_back()
            .ok_or(AoCError::new("must have at least two lines"))?;

        assert_ne!(frist, last, "sanity check");

        let pins = lines
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => 0,
                        '#' => 1,
                        _ => unreachable!("all symbols are . or #"),
                    })
                    .collect_vec()
            })
            .fold([0; 5], |mut acc, line| {
                for i in 0..5 {
                    acc[i] += line[i];
                }
                acc
            });
        Ok(match frist.chars().next().unwrap() {
            '#' => Self::Lock(pins),
            '.' => Self::Key(pins),
            _ => unreachable!(),
        })
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let (locks, keys): (Vec<_>, Vec<_>) = InputPartitioner::new(data, |line| !line.is_empty())
        .map(|lines| Pinnable::new(lines).unwrap())
        .partition_map(|pinnable| match pinnable {
            Pinnable::Lock(pins) => Either::Left(pins),
            Pinnable::Key(pins) => Either::Right(pins),
        });
    let ret: usize = locks
        .into_iter()
        .map(|lock_pins| {
            keys.iter()
                .filter(|key_pins| {
                    lock_pins
                        .iter()
                        .zip(key_pins.iter())
                        .all(|(a, b)| a + b <= 5)
                })
                .count()
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "25",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None,
});
