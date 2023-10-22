use std::collections::HashSet;
use std::str::FromStr;

use crate::{Coord2D, Direction};

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::North),
            "R" => Ok(Direction::East),
            "D" => Ok(Direction::South),
            "L" => Ok(Direction::West),
            _ => Err(format!("Unknown direction {s}")),
        }
    }
}

struct Instruction {
    dir: Direction,
    amt: u64,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = match s.split_once(' ') {
            Some((dir, amt)) => (dir, amt),
            None => return Err(format!("Incorrectly formatted instruction {s}")),
        };
        Ok(Instruction {
            dir: dir.parse()?,
            amt: amt.parse::<u64>().map_err(|e| e.to_string())?,
        })
    }
}

impl Instruction {
    fn coordinate(&self) -> impl Iterator<Item = Coord2D> {
        let coord = self.dir.into();
        (0..self.amt).map(move |_| coord)
    }
}

fn magic_clamp(val: i32) -> i32 {
    if val > 1 {
        1
    } else if val < -1 {
        -1
    } else {
        val
    }
}

pub fn main(data: crate::DataIn) -> String {
    let mut head_pos = Coord2D { x: 0, y: 0 };
    let mut tail_pos = head_pos;

    let mut visited: HashSet<Coord2D> = HashSet::new();
    visited.insert(tail_pos);

    for line in data {
        let instr: Instruction = line.parse().unwrap();
        for coord in instr.coordinate() {
            head_pos = head_pos + coord;
            let dist = head_pos - tail_pos;
            if dist.x.abs() > 1 || dist.y.abs() > 1 {
                // eyy I move a da tail
                if dist.x == 0 || dist.y == 0 {
                    // we can cheat beacuse we know the length is 2
                    tail_pos = tail_pos + (dist / 2).unwrap();
                } else {
                    // fuck idk
                    tail_pos = tail_pos
                        + Coord2D {
                            x: magic_clamp(dist.x),
                            y: magic_clamp(dist.y),
                        }
                }
                visited.insert(tail_pos);
            }
        }
    }
    visited.len().to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "9",
    func: main,
});
