// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;

use cached::{UnboundCache, cached_key};
use itertools::Itertools;

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut adaptors: Vec<u64> = data.map(|line| line.parse()).try_collect()?;
    adaptors.push(0);
    adaptors.sort();
    adaptors.push(adaptors.last().unwrap() + 3);

    let differences = adaptors
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .counts();
    log::debug!("differences: {differences:?}");
    // sanity check
    assert!(differences.contains_key(&1));
    assert!(differences.contains_key(&3));
    assert_eq!(differences.len(), 2);
    let ret = differences[&1] * differences[&3];
    Ok(ret.to_string())
}

cached_key! {
    PERMUTATE: UnboundCache<(u64, usize), u64> = UnboundCache::with_capacity(100);
    // Include the length for the examples
    Key = { (jolts, compatible.len()) };
    fn permutate(jolts: u64, compatible: &HashMap<u64, Vec<u64>>) -> u64 = {
        let my_compat = &compatible[&jolts];
        log::debug!("Hello I'm {jolts} and my friends are {my_compat:?}");
        if my_compat.is_empty() {
            return 1;
        } else {
            my_compat
                .iter()
                .map(|jolts| permutate(*jolts, compatible))
                .sum()
        }
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut adaptors: Vec<u64> = data.map(|line| line.parse()).try_collect()?;
    adaptors.push(0);
    adaptors.sort();
    adaptors.push(adaptors.last().unwrap() + 3);

    let compatible: HashMap<u64, Vec<u64>> = adaptors
        .iter()
        .copied()
        .enumerate()
        .map(|(i, my_jolts)| {
            (
                my_jolts,
                adaptors[i + 1..]
                    .iter()
                    .copied()
                    .filter(|jolts| (jolts - my_jolts) <= 3)
                    .collect(),
            )
        })
        .collect();

    log::debug!("compo {compatible:?} ");

    let ret = permutate(0, &compatible);
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2020",
    day: "10",
    part_1: crate::AoCPart {
        main: part_1,
        example: |data| crate::partitioned_example(data, part_1)
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::partitioned_example(data, part_2)
    })
});
