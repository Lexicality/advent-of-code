// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::cmp;
use std::fmt::Display;

use itertools::Itertools;

use crate::{CommonGrid, Coord2D, Coordinate, Direction, InfGrid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum GridData {
    #[default]
    Air,
    Rock,
    Sand,
}

impl Display for GridData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
        }
    }
}

const SAND_STEPS: [Coord2D; 3] = [
    Coord2D { x: 0, y: 1 },
    Coord2D { x: -1, y: 1 },
    Coord2D { x: 1, y: 1 },
];

fn get_next_sand(pos: Coord2D, floor_y: i32, grid: &InfGrid<GridData>) -> Option<Coord2D> {
    for step in SAND_STEPS {
        let target = pos + step;
        if target.y >= floor_y {
            return None;
        }
        let contents = grid.get(&target);
        match contents {
            None => return Some(target),
            Some(GridData::Air) => return Some(target),
            _ => (),
        }
    }
    None
}

fn settle_sand(sand_start: Coord2D, floor_y: i32, grid: &InfGrid<GridData>) -> Option<Coord2D> {
    if grid.get(&sand_start).is_some() {
        return None;
    }
    let mut sand = sand_start;
    loop {
        let behaviour = get_next_sand(sand, floor_y, grid);
        match behaviour {
            Some(pos) => sand = pos,
            None => return Some(sand),
        }
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut maxy = 0;
    let rocks: Vec<Vec<Coord2D>> = data
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let coord: Coord2D = coord.parse().unwrap();
                    maxy = cmp::max(maxy, coord.y);
                    coord
                })
                .collect()
        })
        .collect();
    let mut grid = InfGrid::<GridData>::new();

    for rock in rocks.iter() {
        for (start, end) in rock.iter().tuple_windows() {
            let len = start.distance(end);
            let dir: Coord2D = {
                // Silly way to flatten & validate the direction
                let dir: Direction = (*end - *start).try_into().unwrap();
                dir.into()
            };
            let mut pos = *start;
            for _ in 0..=len {
                grid.set(pos, GridData::Rock);
                pos += dir;
            }
        }
    }
    println!("{grid}\n=====\n");

    let sand_start = Coord2D { x: 500, y: 0 };
    let floor_y = maxy + 2;
    let mut i = 0;
    loop {
        let sandpos = settle_sand(sand_start, floor_y, &grid);
        match sandpos {
            Some(pos) => {
                grid.set(pos, GridData::Sand);
            }
            None => break,
        }
        i += 1;
        // println!("{grid}");
    }

    println!("{grid}");

    Ok(i.to_string())
}

inventory::submit!(crate::AoCDay::mew("2022", "14", main));
