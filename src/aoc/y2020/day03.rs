// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use aoc_macros::VoidState;

use crate::{CharGrid, CommonGrid, Coord2D, SparseGrid, symbols};

#[derive(VoidState)]
enum GridState {
    #[void]
    Snow,
    Tree,
}

impl TryFrom<char> for GridState {
    type Error = crate::AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Snow,
            '#' => Self::Tree,
            _ => return Err(Self::Error::new_from_char(value)),
        })
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Snow => symbols::VOID,
            Self::Tree => symbols::TREE,
        }
        .fmt(f)
    }
}

fn toboggonate(grid: &SparseGrid<GridState>, dir: Coord2D) -> usize {
    let mut pos = Coord2D { x: 0, y: 0 };
    let mut trees = 0;
    let max = grid.max_key();
    while pos.y <= max.y {
        if let Some(GridState::Tree) = grid.get(&pos) {
            log::trace!("Hit a tree at {pos}");
            trees += 1
        } else {
            log::trace!("safe at {pos}");
        }
        pos += dir;
        pos.x %= max.x + 1;
    }
    log::debug!("Fell of the end of the map at {pos}");
    trees
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: SparseGrid<GridState> = SparseGrid::new_from_chars(data)?;
    log::debug!("\n{grid:#}");
    let ret = toboggonate(&grid, Coord2D { x: 3, y: 1 });
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: SparseGrid<GridState> = SparseGrid::new_from_chars(data)?;
    log::debug!("\n{grid:#}");
    let ret: usize = [
        Coord2D { x: 1, y: 1 },
        Coord2D { x: 3, y: 1 },
        Coord2D { x: 5, y: 1 },
        Coord2D { x: 7, y: 1 },
        Coord2D { x: 1, y: 2 },
    ]
    .into_iter()
    .map(|coord| toboggonate(&grid, coord))
    .product();

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2020",
    day: "3",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
