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

#[derive(Debug)]
enum Part {
    Part1,
    Part2,
}

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
    let day = day.strip_prefix("0").unwrap_or(day);
    let year = day_path
        .parent()
        .and_then(|parent| parent.iter().next_back())
        .and_then(|dir| dir.to_str())
        .and_then(|year| year.strip_prefix("y"))
        .unwrap();

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
        func(data.into_iter()).unwrap();
    }
}
