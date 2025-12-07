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

use crate::{AoCError, symbols};

pub trait VoidState: Default {
    fn is_void(&self) -> bool;
}

/// Easy catchall gridstate for most problems
#[derive(Debug, Clone, Copy, PartialEq, Eq, VoidState)]
pub enum GridState {
    #[void]
    Void,
    Block,
    Start,
    End,
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => symbols::VOID,
            Self::Block => symbols::BLOCK,
            Self::Start => symbols::START,
            Self::End => symbols::END,
        }
        .fmt(f)
    }
}

impl TryFrom<char> for GridState {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Void,
            '#' => Self::Block,
            'S' => Self::Start,
            'E' => Self::End,
            _ => return Err(AoCError::new_from_char(value)),
        })
    }
}
