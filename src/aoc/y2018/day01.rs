// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashSet;

use itertools::Itertools;

use crate::AoCError;

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret: i64 = data
        .map(|line| line.parse())
        .try_collect::<_, Vec<i64>, _>()
        .map_err(AoCError::new_from_parseerror)?
        .into_iter()
        .sum();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let changes: Vec<i64> = data
        .map(|line| line.parse())
        .try_collect()
        .map_err(AoCError::new_from_parseerror)?;

    let mut seen = HashSet::with_capacity(changes.len());
    let mut current = 0;
    for change in changes.into_iter().cycle() {
        current += change;
        if !seen.insert(current) {
            return Ok(current.to_string());
        }
    }
    unreachable!()
}

inventory::submit!(crate::AoCDay {
    year: "2018",
    day: "1",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: |data| crate::partitioned_example(data, part_1)
    }),
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::partitioned_example(data, part_2)
    }),
});
