// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use lazy_static::lazy_static;
use regex::Regex;

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    }

    let mut ret = 0;
    for line in data {
        for [a, b] in LINE_RE.captures_iter(&line).map(|c| c.extract().1) {
            let a: u64 = a.parse()?;
            let b: u64 = b.parse()?;
            ret += a * b;
        }
    }
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    }

    let mut ret = 0;
    let mut dont = false;
    for line in data {
        for capture in LINE_RE.captures_iter(&line) {
            if &capture[0] == "don't()" {
                dont = true;
            } else if &capture[0] == "do()" {
                dont = false;
            } else if !dont {
                let a: u64 = capture[1].parse()?;
                let b: u64 = capture[2].parse()?;
                ret += a * b;
            }
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "3",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
