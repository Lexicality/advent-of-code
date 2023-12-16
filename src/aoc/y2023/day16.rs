use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use itertools::Itertools;

use crate::{AoCError, Coord2D, Direction, Grid};

enum GridState {
    Void,
    MirrorA, // /
    MirrorB, // \
    SplitterHorizontal,
    SplitterVertical,
}

impl TryFrom<char> for GridState {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Void),
            '/' => Ok(Self::MirrorA),
            '\\' => Ok(Self::MirrorB),
            '|' => Ok(Self::SplitterVertical),
            '-' => Ok(Self::SplitterHorizontal),
            _ => Err(AoCError::new(format!("Unknown character {value}"))),
        }
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => '·',
            Self::MirrorA => '╱',
            Self::MirrorB => '╲',
            Self::SplitterVertical => '│',
            Self::SplitterHorizontal => '─',
        }
        .fmt(f)
    }
}

impl GridState {
    fn cast(&self, incoming: Direction) -> Vec<Direction> {
        match self {
            Self::Void => vec![incoming],
            Self::MirrorA => match incoming {
                Direction::North => vec![Direction::East],
                Direction::East => vec![Direction::North],
                Direction::South => vec![Direction::West],
                Direction::West => vec![Direction::South],
            },
            Self::MirrorB => match incoming {
                Direction::North => vec![Direction::West],
                Direction::East => vec![Direction::South],
                Direction::South => vec![Direction::East],
                Direction::West => vec![Direction::North],
            },
            Self::SplitterVertical => match incoming {
                Direction::North | Direction::South => vec![incoming],
                _ => vec![Direction::North, Direction::South],
            },
            Self::SplitterHorizontal => match incoming {
                Direction::East | Direction::West => vec![incoming],
                _ => vec![Direction::East, Direction::West],
            },
        }
    }
}

type LightStep = (Coord2D, Direction);

pub fn main(data: crate::DataIn) -> String {
    let grid: Grid<GridState> = Grid::new_from_lines(
        data.map(|line| line.chars().map(|c| c.try_into().unwrap()).collect_vec()),
    );
    println!("{grid}");
    let grid_cap = grid.grid.capacity();
    let mut big_steppe: VecDeque<LightStep> = VecDeque::with_capacity(grid_cap / 2);
    big_steppe.push_back((Coord2D { x: 0, y: 0 }, Direction::East));
    let mut seen: HashSet<LightStep> = HashSet::with_capacity(grid_cap);
    while let Some(step) = big_steppe.pop_front() {
        seen.insert(step);
        let (pos, dir) = step;
        big_steppe.extend(
            grid.get(&pos)
                .unwrap()
                .cast(dir)
                .into_iter()
                .map(|dir| (pos + dir.into(), dir))
                .filter(|step| grid.check_coord(&step.0) && !seen.contains(step)),
        )
    }
    seen.into_iter()
        .map(|(pos, _)| pos)
        .unique()
        .count()
        .to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "16",
    func: main,
    example_func: None,
});
