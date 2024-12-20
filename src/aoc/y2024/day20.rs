use std::fmt::Display;

use itertools::Itertools;

// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.
use crate::utils::astar::{a_star, AStarProvider};
use crate::{symbols, AoCError, AoCResult, CharGrid, Coord2D, Coordinate, Grid};

const TIME_TO_SAVE: usize = 74;

type AStarID = Coord2D;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum GridState {
    Empty,
    Wall,
    Start,
    End,
}

impl TryFrom<char> for GridState {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => return Err(AoCError::new_from_char(value)),
        })
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Empty => symbols::VOID,
            GridState::Wall => symbols::BLOCK,
            GridState::Start => 'S',
            GridState::End => 'E',
        }
        .fmt(f)
    }
}

#[derive(Debug)]
struct AStarImpl {
    grid: Grid<GridState>,
    start: Coord2D,
    end: Coord2D,
}

impl AStarImpl {
    fn new_from_chars(data: crate::DataIn) -> AoCResult<Self> {
        let grid = Grid::new_from_chars(data)?;
        let start = grid
            .find(|(_, v)| matches!(v, GridState::Start))
            .ok_or(AoCError::new("must have a start"))?;
        let end = grid
            .find(|(_, v)| matches!(v, GridState::End))
            .ok_or(AoCError::new("must have an end"))?;

        Ok(Self { grid, start, end })
    }

    fn get_start(&self) -> AStarID {
        self.start
    }
}

impl Display for AStarImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.fmt(f)
    }
}

impl AStarProvider for AStarImpl {
    type IDType = AStarID;

    fn get_neighbours(&self, coord: &Self::IDType) -> Box<dyn Iterator<Item = Self::IDType> + '_> {
        Box::new(
            self.grid
                .get_neighbour_coords_filtered(*coord, false, |_, state| {
                    matches!(state, GridState::Empty | GridState::End)
                }),
        )
    }

    fn heuristic(&self, coord: &Self::IDType) -> u64 {
        self.end.distance(coord).try_into().unwrap()
    }

    fn cost(&self, _coord: &Self::IDType) -> u64 {
        1
    }

    fn is_end(&self, coord: &Self::IDType) -> bool {
        coord == &self.end
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let provider = AStarImpl::new_from_chars(data)?;
    let start = provider.get_start();
    let route = a_star(provider, start);
    assert!(!route.is_empty());
    let ret: usize = route
        .iter()
        .enumerate()
        .map(|(i, coord)| {
            route
                .iter()
                .enumerate()
                .skip(i + TIME_TO_SAVE)
                .filter_map(|(j, other)| {
                    let distance = coord.distance(other);
                    if (2..=20).contains(&distance) {
                        // Some((j, distance))
                        Some(j - i - (distance as usize))
                    } else {
                        None
                    }
                })
                .sorted()
                .inspect(|v| println!("{v}"))
                .filter(|v| *v >= TIME_TO_SAVE)
                // .inspect(|(j, distance)| println!("{}", j - i - (*distance as usize)))
                // I'm not entirely sure why this is necessary but it is
                // .filter(|(j, distance)| (j - i - (*distance as usize)) + 1 >= TIME_TO_SAVE)
                .count()
        })
        .inspect(|v| {
            if *v > 0 {
                println!("   {v}")
            }
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "20", main));
