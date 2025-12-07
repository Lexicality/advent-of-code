// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::cmp::Ordering;
use std::ops::Not;

use itertools::Itertools;

use crate::AoCError;

fn locate_fault(numbers: &[u64], window: usize) -> crate::AoCResult<u64> {
    numbers
        .iter()
        .copied()
        .enumerate()
        .skip(window)
        .find_map(|(i, number)| {
            numbers[i - window..i]
                .iter()
                .array_combinations::<2>()
                .any(|[a, b]| a + b == number)
                .not()
                .then_some(number)
        })
        .ok_or(AoCError::new("Couldn't find the dodgy number!"))
}

pub fn part_1(data: crate::DataIn, window: usize) -> crate::AoCResult<String> {
    let numbers: Vec<_> = data.map(|line| line.parse()).try_collect()?;
    let ret = locate_fault(&numbers, window)?;
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn, window: usize) -> crate::AoCResult<String> {
    let numbers: Vec<_> = data.map(|line| line.parse()).try_collect()?;
    let oddo = locate_fault(&numbers, window)?;

    let max_num = numbers.len();

    // I bet we can get away with being dumb here
    let ret = (0..max_num)
        .find_map(|i| {
            // we shouldn't get here, but just in case
            for j in i + 1..max_num {
                let subnums = &numbers[i..j];
                let sum: u64 = subnums.iter().sum();
                match sum.cmp(&oddo) {
                    Ordering::Less => continue,
                    Ordering::Equal => {
                        return Some(subnums.iter().min().unwrap() + subnums.iter().max().unwrap());
                    }
                    Ordering::Greater => break,
                }
            }
            None
        })
        .ok_or(AoCError::new("Couldn't find matching range :("))?;
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2020",
    day: "9",
    part_1: crate::AoCPart {
        main: |data| part_1(data, 25),
        example: |data| part_1(data, 5)
    },
    part_2: Some(crate::AoCPart {
        main: |data| part_2(data, 25),
        example: |data| part_2(data, 5)
    }),
});
