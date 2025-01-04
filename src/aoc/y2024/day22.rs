// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

use crate::AoCError;

type Aaaa = u64;

const PRUNE: Aaaa = 16777216;
const ITERATIONS: usize = 2000;

fn evolvinate(mut value: Aaaa) -> Aaaa {
    value ^= value * 64;
    value %= PRUNE;
    value ^= value / 32;
    value %= PRUNE;
    value ^= value * 2048;
    value %= PRUNE;
    value
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret = data
        .map(|line| line.parse().map_err(AoCError::new_from_parseerror))
        .map_ok(|num| (0..ITERATIONS).fold(num, |value, _| evolvinate(value)))
        .try_fold(0, |acc, value| value.map(|value| acc + value))?;
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "22",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None,
});
