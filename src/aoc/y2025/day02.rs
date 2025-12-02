// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::ops::RangeInclusive;

use itertools::Itertools;
use num::Integer;

use crate::{AoCError, AoCResult};

fn rangify(data: crate::DataIn) -> AoCResult<Vec<RangeInclusive<u64>>> {
    // damn references
    let lines = data.collect_vec();
    lines
        .iter()
        .flat_map(|line| line.split(','))
        .map(|raw| {
            raw.split_once('-')
                .ok_or_else(|| AoCError::new(format!("Line {raw} is missing a -!")))
                .and_then(|(start, end)| Ok((start.parse()?)..=(end.parse()?)))
        })
        .try_collect()
        .inspect(|ranges| log::debug!("Collected ranges: {ranges:?}"))
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret: u64 = rangify(data)?
        .into_iter()
        .flat_map(|range| {
            range.filter(|num| {
                let numstr = num.to_string();
                // all invalid ids are a pair of numbers, so they must be even
                if numstr.len().is_odd() {
                    return false;
                }
                // safety: this string is made of numbers which are all going to be ascii
                // characters unless something has gone catastrophically wrong
                let (start, end) = numstr.split_at(numstr.len() / 2);
                start == end
            })
        })
        .inspect(|serial| log::debug!("Found invalid serial number {serial}"))
        .sum();

    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret: u64 = rangify(data)?
        .into_iter()
        .flat_map(|range| {
            range.filter(|num| {
                // safety: this string is made of numbers which are all going to be ascii
                // characters unless something has gone catastrophically wrong
                let numstr = num.to_string();
                let len = numstr.len();
                let max_len = len / 2;

                (1..=max_len)
                    // Filter out any number that can't make valid windows
                    .filter(|size| len % size == 0)
                    .any(|size| {
                        let substr = &numstr[0..size];
                        (size..=len - size)
                            .step_by(size)
                            .all(|start| &numstr[start..start + size] == substr)
                    })
            })
        })
        .inspect(|serial| log::debug!("Found invalid serial number {serial}"))
        .sum();

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "2",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});
