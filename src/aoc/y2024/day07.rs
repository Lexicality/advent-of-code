use std::collections::HashMap;

use itertools::Itertools;

use crate::AoCError;

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
            if nums.len() > 1 {
                return None;
            }
            Some(result)
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "7", main));
