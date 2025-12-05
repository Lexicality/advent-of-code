// Copyright (c) 2025 Lexi Robinson
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
    let nums: Vec<u64> = data.map(|line| line.parse()).try_collect()?;
    let num_set: HashSet<_> = nums.iter().copied().collect();

    let ret = nums
        .into_iter()
        .find_map(|num| {
            let target = 2020 - num;
            if num_set.contains(&target) {
                Some(num * target)
            } else {
                None
            }
        })
        .ok_or(AoCError::new("No matching numbers found"))?;
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let nums: Vec<u64> = data.map(|line| line.parse()).try_collect()?;
    let num_set: HashSet<_> = nums.iter().copied().collect();

    let ret = nums
        .iter()
        .enumerate()
        .find_map(|(i, num1)| {
            nums.iter()
                .skip(i + 1)
                .filter(|num2| **num2 + *num1 < 2020)
                .find_map(|num2| {
                    let target = 2020 - num1 - num2;
                    if num_set.contains(&target) {
                        Some(num1 * num2 * target)
                    } else {
                        None
                    }
                })
        })
        .ok_or(AoCError::new("No matching numbers found"))?;
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2020",
    day: "1",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});
