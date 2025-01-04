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
    let mut twos = 0;
    let mut threes = 0;
    for line in data {
        let counts = line.chars().counts();
        for count in counts.values().copied().unique() {
            if count == 2 {
                twos += 1;
            } else if count == 3 {
                threes += 1;
            }
        }
    }

    let ret = twos * threes;
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    for (line1, line2) in data.sorted().tuple_windows() {
        let mut beep = false;

        if line1.chars().zip(line2.chars()).all(|(a, b)| {
            if a != b {
                if beep {
                    false
                } else {
                    beep = true;
                    true
                }
            } else {
                true
            }
        }) {
            return Ok(line1
                .chars()
                .zip(line2.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect());
        }
    }
    unreachable!()
}

inventory::submit!(crate::AoCDay {
    year: "2018",
    day: "2",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
