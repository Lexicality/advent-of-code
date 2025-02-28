// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::cmp;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{utils::bigcoord2d::BigCoord2D, Coordinate};

#[derive(Debug)]
struct Reading {
    sensor: BigCoord2D,
    beacon: BigCoord2D,
    distance: i64,
}

impl Reading {
    pub fn parse(line: String) -> Reading {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
            )
            .unwrap();
        }
        let matches = RE.captures(&line).expect("Line should match regex?");
        let sensor = BigCoord2D {
            x: matches[1].parse().unwrap(),
            y: matches[2].parse().unwrap(),
        };
        let beacon = BigCoord2D {
            x: matches[3].parse().unwrap(),
            y: matches[4].parse().unwrap(),
        };
        Reading {
            sensor,
            beacon,
            distance: sensor.distance(&beacon),
        }
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let readings: Vec<Reading> = data.map(Reading::parse).collect();
    let (min, max, maxdist) = readings.iter().fold(
        (BigCoord2D::MAX, BigCoord2D::MIN, 0),
        |(min, max, maxdist), reading| {
            (
                min.get_min(&reading.sensor).get_min(&reading.beacon),
                max.get_max(&reading.sensor).get_max(&reading.beacon),
                cmp::max(maxdist, reading.distance),
            )
        },
    );

    let y = 2_000_000;

    let count = (min.x - maxdist..=max.x + maxdist)
        .filter(|x| {
            let coord = BigCoord2D { x: *x, y };
            readings.iter().any(|reading| {
                reading.sensor.distance(&coord) <= reading.distance && coord != reading.beacon
            })
        })
        .count();

    Ok(count.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "15",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None,
});
