// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use crate::utils::astar::{a_star, AStarProvider};
use crate::{symbols, AoCResult, CommonGrid, Coord2D, Coordinate, Direction, Grid};

#[derive(Debug, Default)]
enum GridState {
    #[default]
    Void,
    Corrupted(usize),
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Void => symbols::VOID,
            GridState::Corrupted(_) => symbols::BLOCK,
        }
        .fmt(f)
    }
}

#[derive(Debug)]
struct AStarImpl {
    grid: Grid<GridState>,
    end: Coord2D,
}

type AStarID = (Coord2D, usize);

impl AStarImpl {
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

impl Display for AStarImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.fmt(f)
    }
}

impl AStarProvider for AStarImpl {
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

fn puzzle(data: crate::DataIn, width: u32, iterations: usize) -> AoCResult<String> {
    let provider = AStarImpl::new(data, width, iterations)?;
    let start = provider.get_start();
    let res = a_star(provider, start);
    let ret = res.len();
    Ok(ret.to_string())
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    puzzle(data, 71, 1024)
}

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    puzzle(data, 7, 12)
}

inventory::submit!(crate::AoCDay::mew_with_example(
    "2024",
    "18",
    main,
    main_example
));
