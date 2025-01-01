// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, AoCResult};

type GuardID = u32;
type GuardSleeps = [u32; 60];
type GuardData = (GuardID, GuardSleeps);

fn parse_guards(data: crate::DataIn) -> AoCResult<Vec<GuardData>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^\[[0-9-]+ ..:(\d\d)\] (Guard #(\d+) begins shift|wakes up|falls asleep)$"
        )
        .unwrap();
    }

    let mut guards: HashMap<GuardID, GuardSleeps> = HashMap::new();

    let mut current_guard_id: GuardID = GuardID::MAX; // mildly unsound but whatever
    let mut slept_at: Option<usize> = None;

    for line in data.sorted() {
        let matches = RE
            .captures(&line)
            .ok_or_else(|| AoCError::new(format!("input {line} does not match regex")))?;

        if matches[2].starts_with("Guard") {
            current_guard_id = matches[3].parse().map_err(AoCError::new_from_parseerror)?;
            continue;
        } else if &matches[2] == "falls asleep" {
            let ret = slept_at.replace(matches[1].parse().map_err(AoCError::new_from_parseerror)?);
            assert!(ret.is_none(), "fell asleep twice?");
            continue;
        }
        let mins = guards.entry(current_guard_id).or_insert([0; 60]);
        let slept_at = slept_at.take().expect("woke up twice?");
        let woke_at = matches[1].parse().map_err(AoCError::new_from_parseerror)?;
        for min in &mut mins[slept_at..woke_at] {
            *min += 1;
        }
    }

    Ok(guards.into_iter().collect_vec())
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let guards = parse_guards(data)?;
    let sleepiest_soldier = guards
        .into_iter()
        .sorted_by_cached_key(|guard| guard.1.iter().sum::<u32>())
        .next_back()
        .unwrap();

    let ret = usize::try_from(sleepiest_soldier.0).unwrap()
        * sleepiest_soldier
            .1
            .into_iter()
            .enumerate()
            .sorted_by_cached_key(|(_, n)| *n)
            .next_back()
            .unwrap()
            .0;
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let guards = parse_guards(data)?;
    let sleepiest_soldier = guards
        .into_iter()
        .sorted_by_cached_key(|guard| guard.1.iter().max().copied().unwrap())
        .next_back()
        .unwrap();
    let ret = usize::try_from(sleepiest_soldier.0).unwrap()
        * sleepiest_soldier
            .1
            .into_iter()
            .enumerate()
            .sorted_by_cached_key(|(_, n)| *n)
            .next_back()
            .unwrap()
            .0;
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2018",
    day: "4",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1
    }),
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
