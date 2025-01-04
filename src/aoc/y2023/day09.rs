// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

#[allow(unused_variables)]
pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let lines = data.map(|line| {
        line.split_ascii_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect_vec()
    });

    let mut ret = 0;
    for mut line in lines {
        let mut lasts = vec![line.last().copied().unwrap()];
        let mut iterations = 0;
        while !line.iter().all(|n| *n == 0) {
            iterations += 1;
            line = line
                .into_iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();
            lasts.push(line.last().copied().unwrap());
        }
        ret += lasts.into_iter().sum::<i32>();
    }

    Ok(ret.to_string())
}

#[allow(unused_variables)]
pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let lines = data.map(|line| {
        line.split_ascii_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect_vec()
    });

    let mut ret = 0;
    for mut line in lines {
        let mut firsts = vec![line.first().copied().unwrap()];
        while !line.iter().all(|n| *n == 0) {
            line = line
                .into_iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();
            firsts.push(line.first().copied().unwrap());
        }
        ret += firsts
            .into_iter()
            .rev()
            .reduce(|acc, value| value - acc)
            .unwrap();
    }

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "9",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
