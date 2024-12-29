// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::{FoldWhile, Itertools};

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    Ok(data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .enumerate()
        .fold_while((0, 0), |(_, sum), (i, v)| {
            let res = sum + v;
            if res >= 0 {
                FoldWhile::Continue((0, res))
            } else {
                FoldWhile::Done((i + 1, 0))
            }
        })
        .into_inner()
        .0
        .to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "1",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::multi_line_example(data, part_2),
    }),
});
