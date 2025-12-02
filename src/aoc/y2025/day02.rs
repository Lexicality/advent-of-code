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

use crate::AoCError;

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    // damn references
    let lines = data.collect_vec();
    let ranges: Vec<RangeInclusive<u64>> = lines
        .iter()
        .flat_map(|line| line.split(','))
        .map(|raw| {
            raw.split_once('-')
                .ok_or_else(|| AoCError::new(format!("Line {raw} is missing a -!")))
                .and_then(|(start, end)| Ok((start.parse()?)..=(end.parse()?)))
        })
        .try_collect()?;

    log::debug!("Collected ranges: {ranges:?}");

    let ret: u64 = ranges
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

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "2",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None
});
