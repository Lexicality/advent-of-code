// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashSet;
use std::str::FromStr;

use crate::{AoCError, Coord2D, Direction};

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
            dir: dir.parse().map_err(|e: AoCError| e.to_string())?,
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
    val.clamp(-1, 1)
}

impl Coord2D {
    fn maybe_move(self, parent: Self) -> Option<Self> {
        let dist = parent - self;
        if !(dist.x.abs() > 1 || dist.y.abs() > 1) {
            return None;
        }
        if dist.x == 0 || dist.y == 0 {
            Some(self + (dist / 2).unwrap())
        } else {
            Some(
                self + Coord2D {
                    x: magic_clamp(dist.x),
                    y: magic_clamp(dist.y),
                },
            )
        }
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut head_pos = Coord2D { x: 0, y: 0 };
    let mut tail_pos = head_pos;

    let mut visited: HashSet<Coord2D> = HashSet::new();
    visited.insert(tail_pos);

    for line in data {
        let instr: Instruction = line.parse().unwrap();
        for coord in instr.coordinate() {
            head_pos += coord;
            let dist = head_pos - tail_pos;
            if dist.x.abs() > 1 || dist.y.abs() > 1 {
                // eyy I move a da tail
                if dist.x == 0 || dist.y == 0 {
                    // we can cheat beacuse we know the length is 2
                    tail_pos += (dist / 2).unwrap();
                } else {
                    // fuck idk
                    tail_pos += Coord2D {
                        x: magic_clamp(dist.x),
                        y: magic_clamp(dist.y),
                    }
                }
                visited.insert(tail_pos);
            }
        }
    }
    Ok(visited.len().to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut head_pos = Coord2D { x: 0, y: 0 };
    let mut midroll: [Coord2D; 8] = [head_pos; 8];
    let mut tail_pos = head_pos;

    let mut visited: HashSet<Coord2D> = HashSet::new();
    visited.insert(tail_pos);

    for line in data {
        // println!("== {line} ==");
        let instr: Instruction = line.parse().unwrap();
        for coord in instr.coordinate() {
            head_pos += coord;
            let mut last = head_pos;
            midroll.iter_mut().for_each(|coord| {
                let beep = coord.maybe_move(last);
                if let Some(yay) = beep {
                    coord.x = yay.x;
                    coord.y = yay.y;
                }
                last = *coord;
            });
            let maybe_tail = tail_pos.maybe_move(last);
            if let Some(new_tail) = maybe_tail {
                tail_pos = new_tail;
                visited.insert(tail_pos);
            }
            // let mut grid = Grid::<char>::new_filled(6, 5, '.');
            // grid.set(tail_pos, '9');
            // for (coord, name) in midroll
            //     .iter()
            //     .zip(['1', '2', '3', '4', '5', '6', '7', '8'])
            //     .rev()
            // {
            //     grid.set(*coord, name);
            // }
            // grid.set(head_pos, 'H');
            // println!("{grid}");
        }
    }
    Ok(visited.len().to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "9",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
