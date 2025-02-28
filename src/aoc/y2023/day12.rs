// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::{Debug, Display};

use itertools::{repeat_n, EitherOrBoth, Itertools};

use crate::AoCError;

#[derive(PartialEq, Eq, Clone, Copy)]
enum SpringState {
    Operational,
    Damaged,
    Mystery,
}

impl TryFrom<char> for SpringState {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Damaged),
            '.' => Ok(Self::Operational),
            '?' => Ok(Self::Mystery),
            _ => Err(AoCError::new(format!("Unknown spring state {value}"))),
        }
    }
}

impl Debug for SpringState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(
            &match self {
                Self::Operational => '.',
                Self::Damaged => '#',
                Self::Mystery => '?',
            },
            f,
        )
    }
}

impl Display for SpringState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(
            &match self {
                Self::Operational => '.',
                Self::Damaged => '#',
                Self::Mystery => '?',
            },
            f,
        )
    }
}

fn check_valid(state1: &[SpringState], state2: &[u32]) -> bool {
    let groups = state1.iter().chunk_by(|v| *v);
    groups
        .into_iter()
        .filter(|(key, _)| matches!(key, SpringState::Damaged))
        .map(|(_, values)| (values.count() as u32))
        .zip_longest(state2)
        .all(|aa| match aa {
            EitherOrBoth::Both(a, b) => a == *b,
            _ => false,
        })
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    for line in data {
        let (report, groups) = line.split_once(' ').unwrap();
        let report: Vec<SpringState> = report.chars().map(|c| c.try_into()).try_collect().unwrap();
        let mystery_count = report
            .iter()
            .filter(|ss| matches!(ss, SpringState::Mystery))
            .count();
        let groups: Vec<u32> = groups.split(',').map(|c| c.parse()).try_collect().unwrap();
        // comedy brute force solution
        for mut spring_states in repeat_n(0..=1, mystery_count).multi_cartesian_product() {
            let sub_report = report
                .iter()
                .map(|v| match v {
                    SpringState::Mystery => match spring_states.pop().unwrap() {
                        0 => SpringState::Operational,
                        _ => SpringState::Damaged,
                    },
                    _ => *v,
                })
                .collect_vec();

            let valid = check_valid(&sub_report, &groups);
            if valid {
                ret += 1;
            }
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "12",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None,
});
