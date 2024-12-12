// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use crate::AoCError;
use lazy_static::lazy_static;
use regex::Regex;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
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
                let a: u64 = capture[1].parse().map_err(AoCError::new_from_parseerror)?;
                let b: u64 = capture[2].parse().map_err(AoCError::new_from_parseerror)?;
                ret += a * b;
            }
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "3", main));
