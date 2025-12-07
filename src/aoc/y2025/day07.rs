// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.
use std::collections::HashSet;
use std::fmt::Display;

use aoc_macros::VoidState;

use crate::{AoCError, CharGrid, CommonGrid, Coord2D, Direction, SparseGrid, symbols};

#[allow(unused)]
#[derive(Debug, Clone, Copy, VoidState)]
enum GridState {
    #[void]
    Void,
    Start,
    Splitter,
    Light,
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Void => symbols::VOID,
            GridState::Start => symbols::START,
            GridState::Splitter => '🯊',
            GridState::Light => symbols::SHADE_LIGHT,
        }
        .fmt(f)
    }
}

impl TryFrom<char> for GridState {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Void,
            'S' => Self::Start,
            '^' => Self::Splitter,
            _ => return Err(AoCError::new_from_char(value)),
        })
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = SparseGrid::<GridState>::new_from_chars(
        data
            // get rid of the empty lines since they don't do anything
            .filter(|line| !line.chars().all(|c| c == '.')),
    )?;
    log::debug!("grid:\n{grid}");
    let max = grid.max_key();

    let mut ret = 0;
    let _ = (1..=max.y)
        .fold(
            grid.iter()
                .filter(|(_, state)| matches!(state, GridState::Start))
                .map(|(coord, _)| *coord)
                .collect(),
            |beams: HashSet<Coord2D>, y| {
                let new_beams: HashSet<Coord2D> = beams
                    .into_iter()
                    .flat_map(|mut c| {
                        c.y = y;
                        if matches!(grid.get(&c), Some(GridState::Splitter)) {
                            ret += 1; // HACK
                            [
                                Some(c + Direction::East.into()),
                                Some(c + Direction::West.into()),
                            ]
                        } else {
                            [Some(c), None]
                        }
                    })
                    .flatten()
                    .collect();
                new_beams.iter().for_each(|c| {
                    grid.set(*c, GridState::Light);
                });
                new_beams
            },
        )
        .len();
    log::debug!("grid:\n{grid}");
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "7",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None
});
