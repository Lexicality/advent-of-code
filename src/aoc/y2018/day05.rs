// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref PAIRS: Vec<String> = {
        ('a'..='z')
            .flat_map(|c| {
                [
                    [c, c.to_ascii_uppercase()].into_iter().collect(),
                    [c.to_ascii_uppercase(), c].into_iter().collect(),
                ]
            })
            .collect()
    };
}

fn reactionate(mut line: String) -> usize {
    let mut lastlen = 0;
    while lastlen != line.len() {
        lastlen = line.len();
        for pattern in PAIRS.iter() {
            line = line.replace(pattern, "");
        }
    }
    lastlen
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();
    let ret = reactionate(line);
    Ok(ret.to_string())
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();
    let polymer_options = line.chars().map(|c| c.to_ascii_lowercase()).unique();
    let ret = polymer_options
        .map(|char_to_remove| {
            reactionate(line.replace([char_to_remove, char_to_remove.to_ascii_uppercase()], ""))
        })
        .min()
        .unwrap();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2018",
    day: "5",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: |data| crate::multi_line_example(data, part_1),
    }),
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::multi_line_example(data, part_2),
    }),
});
