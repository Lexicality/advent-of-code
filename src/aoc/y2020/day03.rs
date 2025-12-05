// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use crate::{CharGrid, CommonGrid, Coord2D, Grid, symbols};

enum GridState {
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

fn toboggonate(grid: &Grid<GridState>, dir: Coord2D) -> usize {
    let mut pos = Coord2D { x: 0, y: 0 };
    let mut trees = 0;
    loop {
        match grid.get(&pos) {
            Some(GridState::Tree) => trees += 1,
            Some(_) => (),
            None => {
                log::debug!("Fell of the end of the map at {pos}");
                break;
            }
        }
        pos += dir;
        pos.x %= grid.width as i32;
    }

    trees
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: Grid<GridState> = Grid::new_from_chars(data)?;
    log::debug!("\n{grid:#}");
    let ret = toboggonate(&grid, Coord2D { x: 3, y: 1 });
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: Grid<GridState> = Grid::new_from_chars(data)?;
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
