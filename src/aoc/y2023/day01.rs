// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

const NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];
const BACKNUMS: [&str; 18] = [
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn find_first(line: &str, nums: &[&str]) -> usize {
    let mut found_index = usize::MAX;
    let mut found_val = "";

    for num in nums {
        if let Some(index) = line.find(num) {
            if index < found_index {
                found_index = index;
                found_val = num;
                if index == 0 {
                    break;
                }
            }
        }
    }
    if found_val.is_empty() {
        panic!("Didn't find anything?!")
    } else if found_val.len() == 1 && found_val.chars().next().unwrap().is_ascii_digit() {
        found_val.parse().unwrap()
    } else {
        nums.iter().find_position(|a| **a == found_val).unwrap().0 + 1
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut total = 0;
    for line in data {
        let backline: String = line.chars().rev().collect();

        let first = find_first(&line, &NUMBERS);
        let last = find_first(&backline, &BACKNUMS);
        println!("{line} {first}{last}");
        let value = first * 10 + last;
        total += value;
    }
    Ok(total.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "1",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
