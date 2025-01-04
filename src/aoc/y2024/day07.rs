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

use crate::AoCError;

const OPERATIONS: [fn(u64, u64) -> u64; 3] = [
    // Add
    |a: u64, b: u64| a + b,
    // Multiply
    |a: u64, b: u64| a * b,
    // Concat
    |a: u64, b: u64| format!("{a}{b}").parse().unwrap(),
];

fn main(
    data: crate::DataIn,
    calibration_operations: &[fn(u64, u64) -> u64],
) -> crate::AoCResult<String> {
    let calibrations: HashMap<u64, Vec<u64>> = data
        .map(|line| {
            let (result, operations) = line.split_once(": ").unwrap();
            let result = result.parse().map_err(AoCError::new_from_parseerror)?;
            let numbers = operations
                .split(' ')
                .map(str::parse)
                .try_collect()
                .map_err(AoCError::new_from_parseerror)?;
            Ok((result, numbers))
        })
        .try_collect()?;

    let ret: u64 = calibrations
        .into_iter()
        .filter_map(|(result, nums)| {
            let mut nums = nums.into_iter();
            let mut options = vec![nums.next().unwrap()];
            for num in nums {
                options = options
                    .into_iter()
                    .flat_map(|option| {
                        calibration_operations
                            .iter()
                            .map(move |operation| operation(option, num))
                    })
                    .filter(|option| option <= &result)
                    .collect();
            }
            options
                .into_iter()
                .any(|res| res == result)
                .then_some(result)
        })
        .sum();
    Ok(ret.to_string())
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    main(data, &OPERATIONS[0..2])
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    main(data, &OPERATIONS)
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "7",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
