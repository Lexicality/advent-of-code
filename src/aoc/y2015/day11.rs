// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashSet;

use itertools::Itertools;

fn validate_password(password: &[char]) -> bool {
    if password.iter().any(|c| matches!(c, 'i' | 'o' | 'l'))
        || !password
            .iter()
            .map(|c| u32::from(*c))
            .tuple_windows()
            .any(|(a, b, c)| a < b && b < c && (c - a) == 2)
    {
        return false;
    }
    let mut seen = HashSet::with_capacity(23);
    password
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a == b && seen.insert(*a))
        .count()
        >= 2
}

fn increment_password(mut password: Vec<char>) -> Vec<char> {
    for c in password.iter_mut().rev() {
        if *c == 'z' {
            *c = 'a';
        } else {
            *c = char::from_u32(u32::from(*c) + 1).unwrap();
            break;
        }
    }
    password
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut password = data.next().unwrap().chars().collect_vec();
    while !validate_password(&password) {
        password = increment_password(password);
    }
    Ok(password.into_iter().collect())
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut password = data.next().unwrap().chars().collect_vec();
    while !validate_password(&password) {
        password = increment_password(password);
    }
    password = increment_password(password);
    while !validate_password(&password) {
        password = increment_password(password);
    }
    Ok(password.into_iter().collect())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "11",
    part_1: crate::AoCPart {
        main: part_1,
        example: |data| crate::multi_line_example(data, part_1)
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::multi_line_example(data, part_2)
    }),
});

#[cfg(test)]
mod test_validate_password {
    use itertools::Itertools;

    use super::validate_password;

    fn pw(pw: &str) -> Vec<char> {
        pw.chars().collect_vec()
    }

    #[test]
    fn example_1() {
        assert!(!validate_password(&pw("hijklmmn")));
    }

    #[test]
    fn example_2() {
        assert!(!validate_password(&pw("abbceffg")));
    }

    #[test]
    fn example_3() {
        assert!(!validate_password(&pw("abbcegjk")));
    }

    #[test]
    fn example_4() {
        assert!(!validate_password(&pw("abcdefgh")));
        assert!(validate_password(&pw("abcdffaa")));
    }

    #[test]
    fn example_5() {
        assert!(!validate_password(&pw("ghijklmn")));
        assert!(validate_password(&pw("ghjaabcc")));
    }
}

#[cfg(test)]
mod test_increment_password {
    use itertools::Itertools;

    use super::increment_password;

    fn pw(pw: &str) -> Vec<char> {
        pw.chars().collect_vec()
    }

    fn depw(pw: Vec<char>) -> String {
        pw.into_iter().collect()
    }

    #[test]
    fn example_1() {
        let start = pw("abc");
        let end = increment_password(start);
        assert_eq!(depw(end).as_str(), "abd");
    }

    #[test]
    fn example_2() {
        let start = pw("abz");
        let end = increment_password(start);
        assert_eq!(depw(end).as_str(), "aca");
    }

    #[test]
    fn example_3() {
        let start = pw("azz");
        let end = increment_password(start);
        assert_eq!(depw(end).as_str(), "baa");
    }

    #[test]
    fn overflow() {
        let start = pw("zzzzzzzz");
        let end = increment_password(start);
        assert_eq!(depw(end).as_str(), "aaaaaaaa");
    }
}
