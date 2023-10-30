use std::fmt::Display;

use itertools::Itertools;

use crate::{Coord2D, Direction, Grid};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SandBehaviour {
    Move(Coord2D),
    Settle,
    Void,
}

fn get_next_sand(pos: Coord2D, grid: &Grid<GridData>) -> SandBehaviour {
    for step in SAND_STEPS {
        let target = pos + step;
        let contents = grid.get(&target);
        match contents {
            None => return SandBehaviour::Void,
            Some(contents) => {
                if let GridData::Air = contents {
                    return SandBehaviour::Move(target);
                }
            }
        }
    }
    SandBehaviour::Settle
}

fn settle_sand(sand_start: Coord2D, grid: &Grid<GridData>) -> Option<Coord2D> {
    // consistency check
    assert_eq!(grid.get(&sand_start), Some(&GridData::Air));
    let mut sand = sand_start;
    loop {
        let behaviour = get_next_sand(sand, grid);
        match behaviour {
            SandBehaviour::Move(pos) => sand = pos,
            SandBehaviour::Void => return None,
            SandBehaviour::Settle => return Some(sand),
        }
    }
}

pub fn main(data: crate::DataIn) -> String {
    let mut max = Coord2D {
        x: i32::MIN,
        y: i32::MIN,
    };
    let mut min = Coord2D { x: i32::MAX, y: 0 };
    let mut rocks: Vec<Vec<Coord2D>> = data
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let coord = Coord2D::parse(coord);
                    min = min.get_min(&coord);
                    max = max.get_max(&coord);
                    coord
                })
                .collect()
        })
        .collect();
    for rock in rocks.iter_mut() {
        for coord in rock.iter_mut() {
            coord.x -= min.x;
        }
    }
    let grid_size = max - min;
    let mut grid = Grid::<GridData>::new(
        TryInto::<u32>::try_into(grid_size.x).expect("No negative cooords") + 1,
        TryInto::<u32>::try_into(grid_size.y).expect("No negative cooords") + 1,
    );

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
                pos = pos + dir;
            }
        }
    }
    println!("{grid}");

    let sand_start = Coord2D {
        x: 500 - min.x,
        y: 0,
    };
    let mut i = 0;
    loop {
        let sandpos = settle_sand(sand_start, &grid);
        match sandpos {
            Some(pos) => grid.set(pos, GridData::Sand),
            None => break,
        }
        i += 1;
        println!("{grid}");
    }

    println!("{grid}");

    i.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "14",
    func: main,
});
