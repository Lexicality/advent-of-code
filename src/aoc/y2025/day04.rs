// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCError, CharGrid, CommonGrid, Coord2D, Grid, symbols};

enum GridState {
    Empty,
    Roll,
}

impl TryFrom<char> for GridState {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '@' => Self::Roll,
            _ => return Err(AoCError::new_from_char(value)),
        })
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => symbols::VOID,
            Self::Roll => symbols::ROLL,
        }
        .fmt(f)
    }
}

fn get_forkable(grid: &Grid<GridState>) -> impl Iterator<Item = Coord2D> {
    grid.iter()
        .filter(|(coord, state)| {
            matches!(state, GridState::Roll)
                && grid
                    .get_neighbours(**coord, true)
                    .filter(|(_, state)| matches!(state, GridState::Roll))
                    .count()
                    < 4
        })
        .map(|(coord, _)| *coord)
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid = Grid::<GridState>::new_from_chars(data)?;
    log::debug!("{grid}");
    let ret = get_forkable(&grid).count();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::<GridState>::new_from_chars(data)?;
    let mut total_removed = 0;

    loop {
        log::debug!("{grid}");
        let forkable = get_forkable(&grid).collect_vec();
        let count = forkable.len();
        if count == 0 {
            break;
        }
        total_removed += count;
        forkable.into_iter().for_each(|coord| {
            grid.set(coord, GridState::Empty);
        });
    }
    Ok(total_removed.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "4",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});
