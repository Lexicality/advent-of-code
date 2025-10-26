// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use itertools::Itertools;

use crate::utils::astar::{a_star, AStarProvider};
use crate::{symbols, AoCResult, CommonGrid, Coord2D, Coordinate, Direction, Grid};

#[derive(Debug, Default, Clone, Copy)]
#[allow(dead_code)]
enum GridState {
    #[default]
    Void,
    Corrupted(usize),
    Walko,
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Void => symbols::VOID,
            GridState::Corrupted(i) if *i < 12 => symbols::BLOCK,
            GridState::Corrupted(_) => symbols::SHADE_LIGHT,
            GridState::Walko => symbols::BOX,
        }
        .fmt(f)
    }
}

#[derive(Debug)]
struct AStarImplPart1 {
    grid: Grid<GridState>,
    end: Coord2D,
}

type AStarID = (Coord2D, usize);

impl AStarImplPart1 {
    fn new(data: crate::DataIn, width: u32, iterations: usize) -> AoCResult<Self> {
        let mut grid = Grid::new(width, width);

        for (i, coord) in data.enumerate().take(iterations) {
            let coord = coord.parse()?;
            grid.set(coord, GridState::Corrupted(i));
        }

        let end = i32::try_from(width).unwrap() - 1;
        Ok(Self {
            grid,
            end: Coord2D { x: end, y: end },
        })
    }

    fn get_start(&self) -> AStarID {
        (Coord2D { x: 0, y: 0 }, 0)
    }
}

impl Display for AStarImplPart1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.fmt(f)
    }
}

impl AStarProvider for AStarImplPart1 {
    type IDType = AStarID;

    fn get_neighbours(&self, id: &Self::IDType) -> Box<dyn Iterator<Item = Self::IDType> + '_> {
        let (coord, steps) = *id;
        Box::new(
            [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .into_iter()
            .map(move |direction| (coord + direction.to_coord(), steps + 1))
            .filter(move |(coord, _steps)| {
                self.grid.get(coord).is_some_and(|value| match value {
                    GridState::Void => true,
                    GridState::Corrupted(_falls_at) => false, //*_falls_at > steps,
                    GridState::Walko => unreachable!(),
                })
            }),
        )
    }

    fn heuristic(&self, (coord, _): &Self::IDType) -> u64 {
        self.end.distance(coord).try_into().unwrap()
    }

    fn cost(&self, _id: &Self::IDType) -> u64 {
        1
    }

    fn is_end(&self, (coord, _): &Self::IDType) -> bool {
        coord == &self.end
    }
}

fn part_1(data: crate::DataIn, width: u32, iterations: usize) -> AoCResult<String> {
    let provider = AStarImplPart1::new(data, width, iterations)?;
    let start = provider.get_start();
    let res = a_star(provider, start);
    let ret = res.len();
    Ok(ret.to_string())
}

#[derive(Debug, Clone)]
struct AStarImplPart2 {
    grid: Grid<GridState>,
    end: Coord2D,
    initial_iterations: usize,
}

type AStarID2 = Coord2D;

impl AStarImplPart2 {
    fn new(data: crate::DataIn, width: u32, iterations: usize) -> AoCResult<Self> {
        let mut grid = Grid::new(width, width);

        for (i, coord) in data.enumerate() {
            let coord = coord.parse()?;
            grid.set(coord, GridState::Corrupted(i));
        }

        let end = i32::try_from(width).unwrap() - 1;
        Ok(Self {
            grid,
            end: Coord2D { x: end, y: end },
            initial_iterations: iterations,
        })
    }

    fn get_start(&self) -> AStarID2 {
        Coord2D { x: 0, y: 0 }
    }
}

impl Display for AStarImplPart2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.fmt(f)
    }
}

impl AStarProvider for AStarImplPart2 {
    type IDType = AStarID2;

    fn get_neighbours(&self, id: &Self::IDType) -> Box<dyn Iterator<Item = Self::IDType> + '_> {
        let coord = *id;
        Box::new(
            [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .into_iter()
            .map(move |direction| coord + direction.to_coord())
            .filter(move |coord| {
                self.grid.get(coord).is_some_and(|value| match value {
                    GridState::Void => true,
                    GridState::Corrupted(falls_at) => *falls_at >= self.initial_iterations,
                    GridState::Walko => unreachable!(),
                })
            }),
        )
    }

    fn heuristic(&self, coord: &Self::IDType) -> u64 {
        self.end.distance(coord).try_into().unwrap()
    }

    fn cost(&self, _id: &Self::IDType) -> u64 {
        1
    }

    fn is_end(&self, coord: &Self::IDType) -> bool {
        coord == &self.end
    }
}

fn part_2(data: crate::DataIn, width: u32, iterations: usize) -> AoCResult<String> {
    let provider = AStarImplPart2::new(data, width, iterations)?;
    let start = provider.get_start();

    // I'm a genius 🤦
    let bytes: Vec<Coord2D> = provider
        .grid
        .iter()
        .filter_map(|(c, v)| match v {
            GridState::Corrupted(i) => Some((i, c)),
            _ => None,
        })
        .sorted_by_cached_key(|v| v.0)
        .map(|(_, c)| *c)
        .collect();

    // deeply, deeply regretting my API design choices with the a_star function
    for (i, coord) in bytes.into_iter().enumerate().skip(iterations) {
        let mut provider = provider.clone();
        provider.initial_iterations = i + 1;
        // let mut griddo = provider.grid.clone();
        let res = a_star(provider, start);
        if res.is_empty() {
            return Ok(coord.to_string());
        }
        // for coord in res {
        //     griddo.set(coord, GridState::Walko);
        // }
        // println!("{coord}:\n{griddo}");
    }

    unreachable!()
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "18",
    part_1: crate::AoCPart {
        main: |data| part_1(data, 71, 1024),
        example: |data| part_1(data, 7, 12)
    },
    part_2: Some(crate::AoCPart {
        main: |data| part_2(data, 71, 1024),
        example: |data| part_2(data, 7, 12)
    }),
});
