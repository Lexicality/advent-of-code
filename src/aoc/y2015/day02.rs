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
    let ret: u32 = data
        .map(|line| {
            let tmp: Vec<u32> = line.split('x').map(|c| c.parse()).try_collect().unwrap();

            let (l, w, h) = tmp.into_iter().collect_tuple().unwrap();

            let a = l * w;
            let b = w * h;
            let c = l * h;
            let slack = a.min(b).min(c);

            2 * a + 2 * b + 2 * c + slack
        })
        .sum();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret: u32 = data
        .map(|line| {
            let mut tmp: Vec<u32> = line.split('x').map(|c| c.parse()).try_collect().unwrap();
            tmp.sort();
            let (l, w, h) = tmp.into_iter().collect_tuple().unwrap();

            let a = (l, w);
            let b = (w, h);
            let c = (l, h);
            let slack = a.min(b).min(c);

            2 * slack.0 + 2 * slack.1 + l * w * h
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
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
