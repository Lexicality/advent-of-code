use std::collections::HashMap;

use itertools::Itertools;

use crate::AoCError;

const OPERATIONS: [fn(u64, u64) -> u64; 2] = [
    // Add
    |a: u64, b: u64| a + b,
    // Multiply
    |a: u64, b: u64| a * b,
];

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
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
                        OPERATIONS
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

inventory::submit!(crate::AoCDay::mew("2024", "7", main));
