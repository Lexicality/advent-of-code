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
use num::Integer;

use crate::AoCError;

type Aa = u64;

fn main(mut data: crate::DataIn, num_iterations: usize) -> crate::AoCResult<String> {
    let mut stones: HashMap<Aa, usize> = data
        .next()
        .unwrap()
        .split(" ")
        .map(str::parse)
        .map_ok(|v| (v, 1))
        .try_collect()
        .map_err(AoCError::new_from_parseerror)?;

    println!("{stones:?}");

    for _ in 0..num_iterations {
        let stlen = stones.len();
        stones = stones
            .into_iter()
            .fold(HashMap::with_capacity(stlen), |mut acc, (num, count)| {
                if num == 0 {
                    *acc.entry(1).or_default() += count;
                } else {
                    let strval = num.to_string();
                    if strval.len().is_even() {
                        let (a, b) = strval.split_at(strval.len() / 2);
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        *acc.entry(a).or_default() += count;
                        *acc.entry(b).or_default() += count;
                    } else {
                        *acc.entry(num * 2024).or_default() += count;
                    }
                }
                acc
            });
    }

    let ret: usize = stones.values().sum();
    Ok(ret.to_string())
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    main(data, 25)
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    main(data, 75)
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "11",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
