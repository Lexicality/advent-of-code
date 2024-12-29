// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCError, CommonGrid, Coord2D, Direction, FlatGrid, Grid};

#[derive(Debug)]
enum PipeSegment {
    Ground,
    Horizontal,
    Vertical,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Mystery,
}
impl TryFrom<char> for PipeSegment {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::BendNorthEast),
            'J' => Ok(Self::BendNorthWest),
            '7' => Ok(Self::BendSouthWest),
            'F' => Ok(Self::BendSouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Mystery),
            _ => Err(AoCError::new(format!("Unknown pipe segment '{value}'"))),
        }
    }
}

impl Display for PipeSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => '░',
            Self::Vertical => '║',
            Self::Horizontal => '═',
            Self::BendNorthEast => '╚',
            Self::BendNorthWest => '╝',
            Self::BendSouthEast => '╔',
            Self::BendSouthWest => '╗',
            Self::Mystery => '╳',
        }
        .fmt(f)
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
enum Pipe2 {
    #[default]
    Ground,
    Pipe,
    Outside,
}

impl Display for Pipe2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => '░',
            Self::Outside => '╳',
            Self::Pipe => '█',
        }
        .fmt(f)
    }
}

fn directionate(pos: Coord2D, first: Direction, second: Direction) -> Option<(Coord2D, Coord2D)> {
    Some((pos + first.into(), pos + second.into()))
}

impl PipeSegment {
    fn get_neighbours(&self, pos: Coord2D) -> Option<(Coord2D, Coord2D)> {
        match self {
            Self::Vertical => directionate(pos, Direction::North, Direction::South),
            Self::Horizontal => directionate(pos, Direction::East, Direction::West),
            Self::BendNorthEast => directionate(pos, Direction::North, Direction::East),
            Self::BendNorthWest => directionate(pos, Direction::North, Direction::West),
            Self::BendSouthEast => directionate(pos, Direction::South, Direction::East),
            Self::BendSouthWest => directionate(pos, Direction::South, Direction::West),
            _ => None,
        }
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid: Grid<PipeSegment> = Grid::new_from_lines(data.map(|line| {
        line.chars()
            .map(|c| c.try_into().unwrap())
            .collect::<Vec<_>>()
    }));
    // println!("{grid}");

    let start = grid
        .find(|(_, item)| matches!(item, PipeSegment::Mystery))
        .expect("Must have a starting position");

    let mut queue: VecDeque<Coord2D> = grid
        .get_neighbour_coords(start, false)
        .filter(|coord| {
            grid.get(coord)
                .unwrap()
                .get_neighbours(*coord)
                .is_some_and(|(a, b)| a == start || b == start)
        })
        .collect();

    let mut seen: HashSet<Coord2D> = HashSet::with_capacity(grid.len());
    seen.insert(start);

    while let Some(coord) = queue.pop_front() {
        seen.insert(coord);
        let item = grid.get(&coord).unwrap();
        // working on the assumption here that the loop will never go out of bounds
        let (a, b) = item.get_neighbours(coord).expect("loop must be contiguous");
        if !seen.contains(&a) {
            queue.push_back(a);
        }
        if !seen.contains(&b) {
            queue.push_back(b);
        }
    }

    grid.iter_mut()
        .filter(|(coord, _)| !seen.contains(coord))
        .for_each(|(_, item)| *item = PipeSegment::Ground);

    println!("{grid}");

    // Expand the grid
    let width = grid.width;
    let height = grid.height;

    let oneone = Coord2D { x: 1, y: 1 };
    let size = Coord2D { x: 3, y: 3 };

    let mut grid: Grid<Pipe2> = grid
        .into_iter()
        .flat_map(|(coord, item)| {
            let base = coord * size;
            let mut microgrid = Grid::new_filled(3, 3, Pipe2::Ground);
            match item {
                PipeSegment::Ground => (),
                PipeSegment::Mystery => {
                    microgrid.set((0, 1).into(), Pipe2::Pipe);
                    microgrid.set((1, 1).into(), Pipe2::Pipe);
                    microgrid.set((1, 0).into(), Pipe2::Pipe);
                    microgrid.set((1, 2).into(), Pipe2::Pipe);
                    microgrid.set((2, 1).into(), Pipe2::Pipe);
                }
                _ => {
                    let (a, b) = item.get_neighbours(oneone).unwrap();
                    microgrid.set(oneone, Pipe2::Pipe);
                    microgrid.set(a, Pipe2::Pipe);
                    microgrid.set(b, Pipe2::Pipe);
                }
            }
            microgrid
                .into_iter()
                .map(|(coord, item)| (coord + base, item))
                .collect_vec()
        })
        .collect();

    println!("{grid}");

    // Flood fill
    // Conveniently it's always safe to start in the top left because on my input data
    // the top left is never the starting position

    // Fill seen with the pipes so we'll never iterate into them and so I won't have to bother about filtering
    let mut seen: HashSet<_> = grid
        .iter()
        .filter(|(_, item)| matches!(item, Pipe2::Pipe))
        .map(|(coord, _)| coord)
        .copied()
        .collect();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0).into());
    while let Some(coord) = queue.pop_front() {
        if !seen.insert(coord) {
            continue;
        }

        grid.set(coord, Pipe2::Outside);
        queue.extend(
            grid.get_neighbour_coords(coord, false)
                .filter(|coord| !seen.contains(coord)),
        );
    }

    println!("{grid}");

    // shrink the grid again
    let mut ret = 0;
    for x in 0..width as i32 {
        for y in 0..height as i32 {
            let coord = Coord2D::from((x, y)) * size + oneone;
            if matches!(grid.get(&coord), Some(Pipe2::Ground)) {
                ret += 1;
            }
        }
    }

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "10",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::partitioned_example(data, part_2)
    }),
});
