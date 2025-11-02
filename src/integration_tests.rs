// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

#![cfg(test)]

use std::path::PathBuf;
use std::{collections::HashMap, time::Duration};

use itertools::Itertools;
use rstest::{fixture, rstest};

use crate::{AoCData, AoCDay, AoCResult};

type DayMap = HashMap<&'static str, HashMap<&'static str, &'static AoCDay>>;

#[fixture]
#[once]
fn days() -> DayMap {
    (inventory::iter::<AoCDay>)
        .into_iter()
        .map(|day_data| (day_data.year, (day_data.day, day_data)))
        .into_group_map()
        .into_iter()
        .map(|(key, days)| (key, days.into_iter().collect()))
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Part {
    Part1,
    Part2,
}

const BAD_DAYS: [(&str, &str, Part); 9] = [
    ("2015", "04", Part::Part1),
    ("2015", "04", Part::Part2),
    ("2019", "13", Part::Part2),
    ("2019", "19", Part::Part1),
    ("2019", "19", Part::Part2),
    ("2021", "20", Part::Part2),
    ("2021", "22", Part::Part1),
    ("2022", "16", Part::Part1),
    ("2023", "08", Part::Part2),
];

#[rstest]
#[timeout(Duration::from_secs(3))]
fn test_foo(
    days: &DayMap,
    #[files("src/aoc/y*/day*.rs")]
    #[exclude("day00")]
    day_path: PathBuf,
    #[values(Part::Part1, Part::Part2)] part: Part,
) {
    let day = day_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|day| day.strip_prefix("day"))
        .unwrap();
    let year = day_path
        .parent()
        .and_then(|parent| parent.iter().next_back())
        .and_then(|dir| dir.to_str())
        .and_then(|year| year.strip_prefix("y"))
        .unwrap();

    // inefficient but whatever
    for bad_day in BAD_DAYS {
        if bad_day == (year, day, part) {
            return;
        }
    }

    let day = day.strip_prefix("0").unwrap_or(day);

    let data = AoCData::new_from_file(year, day, true).unwrap();

    let day_data = days[year][day];

    let func = match part {
        Part::Part1 => Some(day_data.part_1.example),
        Part::Part2 => day_data.part_2.as_ref().map(|part| part.example),
    };

    if let Some(func) = func
        && !std::ptr::fn_addr_eq(
            func,
            crate::no_example as fn(crate::DataIn) -> AoCResult<String>,
        )
    {
        let res = func(data.into_iter());
        if let Err(err) = res {
            panic!("{err}");
        }
    }
}
