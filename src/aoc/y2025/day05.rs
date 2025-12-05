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

use crate::{AoCError, AoCResult, partition_input};

// This may have to become a u128...
type Num = u64;

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let (ranges, ingredients) = partition_input(data);

    let ranges: Vec<_> = ranges
        .map(|line| -> AoCResult<RangeInclusive<Num>> {
            let (start, end) = line.split_once('-').ok_or_else(|| {
                AoCError::new(format!("Input range {line} doesn't have a - in it?"))
            })?;
            Ok(start.parse()?..=end.parse()?)
        })
        .try_collect()?;

    let ingredients: Vec<_> = ingredients
        .map(|line| -> Result<Num, _> { line.parse() })
        .filter_ok(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .try_collect()?;

    let ret = ingredients.len();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "5",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None
});
