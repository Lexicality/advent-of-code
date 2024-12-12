// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::VecDeque;
use std::fmt::Display;

use crate::{AoCError, CharGrid, CommonGrid, Grid};

struct Height(u32);

impl TryFrom<char> for Height {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self(u32::MAX),
            c if c.is_ascii_digit() => Self(c.to_digit(10).unwrap()),
            _ => return Err(AoCError::new_from_char(value)),
        })
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            c if c < 10 => c.fmt(f),
            _ => '.'.fmt(f),
        }
    }
}

impl Height {
    fn can_walk(&self, from: &Self) -> bool {
        self.0 > from.0 && (self.0 - from.0) == 1
    }

    fn is_trailhead(&self) -> bool {
        self.0 == 0
    }

    fn is_end(&self) -> bool {
        self.0 == 9
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: Grid<Height> = Grid::new_from_chars(data)?;

    let mut ret = 0;
    for trailhead in grid
        .iter()
        .filter(|(_, h)| h.is_trailhead())
        .map(|(c, _)| c)
        .copied()
    {
        // Ungabunga brepth first here we go
        let mut options = VecDeque::with_capacity(grid.len());
        options.push_front(trailhead);

        while let Some(coord) = options.pop_front() {
            let value = grid.get(&coord).unwrap();
            if value.is_end() {
                ret += 1;
                continue;
            }
            options.extend(
                grid.get_neighbour_coords_filtered(coord, false, |_, next| next.can_walk(value)),
            );
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "10", main));
