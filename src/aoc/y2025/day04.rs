// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use crate::{AoCError, CharGrid, Grid, symbols};

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

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid = Grid::<GridState>::new_from_chars(data)?;
    log::debug!("{grid}");
    let ret = grid
        .iter()
        .filter(|(coord, state)| {
            matches!(state, GridState::Roll)
                && grid
                    .get_neighbours(**coord, true)
                    .filter(|(_, state)| matches!(state, GridState::Roll))
                    .count()
                    < 4
        })
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "4",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None
});
