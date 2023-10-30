use std::cmp;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Coord2D {
    pub x: i64,
    pub y: i64,
}

impl Coord2D {
    pub const MAX: Coord2D = Coord2D {
        x: i64::MAX,
        y: i64::MAX,
    };
    pub const MIN: Coord2D = Coord2D {
        x: i64::MIN,
        y: i64::MIN,
    };

    pub fn parse(data: &str) -> Coord2D {
        let (x, y) = data
            .split_once(',')
            .expect("Coordinates must be in the form x,y");
        Coord2D {
            x: x.parse().expect("Failed to parse X:"),
            y: y.parse().expect("Failed to parse Y:"),
        }
    }

    pub fn distance(&self, other: &Coord2D) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    pub fn get_max(&self, other: &Coord2D) -> Coord2D {
        Coord2D {
            x: cmp::max(self.x, other.x),
            y: cmp::max(self.y, other.y),
        }
    }

    pub fn get_min(&self, other: &Coord2D) -> Coord2D {
        Coord2D {
            x: cmp::min(self.x, other.x),
            y: cmp::min(self.y, other.y),
        }
    }
}

#[derive(Debug)]
struct Reading {
    sensor: Coord2D,
    beacon: Coord2D,
    distance: u64,
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
        let sensor = Coord2D {
            x: matches[1].parse().unwrap(),
            y: matches[2].parse().unwrap(),
        };
        let beacon = Coord2D {
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

pub fn main(data: crate::DataIn) -> String {
    let readings: Vec<Reading> = data.map(Reading::parse).collect();
    let (min, max, maxdist) = readings.iter().fold(
        (Coord2D::MAX, Coord2D::MIN, 0),
        |(min, max, maxdist), reading| {
            (
                min.get_min(&reading.sensor).get_min(&reading.beacon),
                max.get_max(&reading.sensor).get_max(&reading.beacon),
                cmp::max(maxdist, reading.distance as i64),
            )
        },
    );

    let y = 2_000_000;

    let count = (min.x - maxdist..=max.x + maxdist)
        .filter(|x| {
            let coord = Coord2D { x: *x, y };
            readings.iter().any(|reading| {
                reading.sensor.distance(&coord) <= reading.distance && coord != reading.beacon
            })
        })
        .count();

    count.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "15",
    func: main,
});
