// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::cmp::Ordering;

fn check_num_part_1(num: u32) -> bool {
    let num = num.to_string();
    if num.len() != 6 {
        return false;
    }
    let mut digits = num.chars().map(|c| c.to_digit(10).unwrap());
    let mut prev = digits.next().unwrap();
    let mut doubles = false;
    for digit in digits {
        match digit.cmp(&prev) {
            Ordering::Less => return false,
            Ordering::Equal => doubles = true,
            Ordering::Greater => (),
        };
        prev = digit;
    }
    doubles
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();
    let (start, end) = line.split_once('-').unwrap();
    let (start, end) = (start.parse().unwrap(), end.parse().unwrap());

    let mut ret = 0;
    for num in start..end {
        if check_num_part_1(num) {
            ret += 1;
        }
    }

    Ok(ret.to_string())
}

fn check_num_part_2(num: u32) -> bool {
    let num = num.to_string();
    if num.len() != 6 {
        return false;
    }
    let mut digits = num.chars().map(|c| c.to_digit(10).unwrap());
    let mut prev = digits.next().unwrap();
    let mut doubles = false;
    let mut repetitions = 0;
    for digit in digits {
        match digit.cmp(&prev) {
            Ordering::Less => return false,
            Ordering::Equal => repetitions += 1,
            Ordering::Greater => {
                if repetitions == 1 {
                    doubles = true;
                }
                repetitions = 0;
            }
        };
        prev = digit;
    }
    if repetitions == 1 {
        doubles = true;
    }
    doubles
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();
    let (start, end) = line.split_once('-').unwrap();
    let (start, end) = (start.parse().unwrap(), end.parse().unwrap());

    let mut ret = 0;
    for num in start..end {
        if check_num_part_2(num) {
            ret += 1;
        }
    }

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "4",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});

#[cfg(test)]
mod test {
    use super::check_num_part_2 as check_num;

    #[test]
    fn adjacent() {
        assert!(check_num(122345));
    }

    #[test]
    fn new_double_rule() {
        assert!(!check_num(111111), "all ones is invalid");
        assert!(check_num(112233), "three pairs is good");
        assert!(check_num(111122), "four-two is fine");
        assert!(!check_num(111222), "three-three is bad");
        assert!(!check_num(122223), "1-4-1 is invalid");
    }

    #[test]
    fn zero() {
        assert!(!check_num(223450));
    }

    #[test]
    fn no_doubs() {
        assert!(!check_num(123789));
    }

    #[test]
    fn len() {
        assert!(!check_num(11378));
    }
}
