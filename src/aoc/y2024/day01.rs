// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let (mut a, mut b): (Vec<_>, Vec<_>) = data
        .map(|line| -> (u64, u64) {
            let (a, b) = line.split_once(' ').expect("line must be splittable");
            (a.trim().parse().unwrap(), b.trim().parse().unwrap())
        })
        .unzip();
    a.sort();
    b.sort();
    assert_eq!(a.len(), b.len(), "lists must have the same length!");
    Ok(a.into_iter()
        .zip_eq(b)
        .map(|(a, b)| a.abs_diff(b))
        .reduce(u64::saturating_add)
        .unwrap()
        .to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let (a, b): (Vec<_>, Vec<_>) = data
        .map(|line| -> (u64, u64) {
            let (a, b) = line.split_once(' ').expect("line must be splittable");
            (a.trim().parse().unwrap(), b.trim().parse().unwrap())
        })
        .unzip();

    let counts = b.into_iter().counts();
    Ok(a.into_iter()
        .map(|a| a * counts.get(&a).map(|b| *b as u64).unwrap_or_default())
        .reduce(u64::saturating_add)
        .unwrap()
        .to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "1",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
