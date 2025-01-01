// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashSet;
use std::fmt::Display;

use itertools::Itertools;

use crate::symbols;
use crate::{AoCError, CharGrid, CommonGrid, Direction, Grid, RotateDirection};

#[derive(Debug, Clone, Copy)]
enum GridState {
    Empty,
    Obstacle,
    Guard,
    Visited(Direction),
    Crossover,
}

impl TryFrom<char> for GridState {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Obstacle),
            '.' => Ok(Self::Empty),
            '^' => Ok(Self::Guard),
            _ => Err(AoCError::new(format!("Unknown value '{value}'"))),
        }
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Empty => symbols::VOID.fmt(f),
            GridState::Obstacle => symbols::BLOCK.fmt(f),
            GridState::Guard => '^'.fmt(f),
            GridState::Visited(direction) => format!("{direction:#}").fmt(f),
            GridState::Crossover => symbols::CROSSOVER.fmt(f),
        }
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::new_from_chars(data)?;
    let mut guardpos = grid
        .find(|(_, v)| matches!(v, GridState::Guard))
        .expect("Guard must exist");
    let starting_position = guardpos;
    println!("{grid:#}");

    const A: Direction = Direction::North;

    grid.set(guardpos, GridState::Visited(A));
    let mut guard_direction = Direction::North;
    loop {
        let nextpos = guardpos + guard_direction.to_coord();
        if nextpos == starting_position && guard_direction == Direction::North {
            println!("Loop!");
            break;
        }

        let Some(nextstate) = grid.get(&nextpos) else {
            println!("They walked off!");
            break;
        };
        if matches!(nextstate, GridState::Obstacle) {
            // Pivot on the spot
            guard_direction = guard_direction.rotate(RotateDirection::Right);
            // Try again
            continue;
        }
        guardpos = nextpos;
        grid.set(guardpos, GridState::Visited(A));
    }

    let ret = grid
        .into_iter()
        .filter(|(_, v)| matches!(v, GridState::Visited(_)))
        .count();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::new_from_chars(data)?;
    let mut guardpos = grid
        .find(|(_, v)| matches!(v, GridState::Guard))
        .expect("Guard must exist");
    let starting_position = guardpos;

    let mut guard_direction = Direction::North;

    loop {
        let nextpos = guardpos + guard_direction.to_coord();
        let Some(nextstate) = grid.get(&nextpos) else {
            break;
        };
        match nextstate {
            GridState::Obstacle => {
                // Pivot on the spot
                guard_direction = guard_direction.rotate(RotateDirection::Right);
                // Try again
                continue;
            }
            GridState::Empty => {
                grid.set(nextpos, GridState::Visited(guard_direction));
            }
            GridState::Visited(_) => {
                grid.set(nextpos, GridState::Crossover);
            }
            _ => (),
        }
        guardpos = nextpos;
    }

    let ret = grid
        .iter()
        .filter(|(_, v)| matches!(v, GridState::Visited(_) | GridState::Crossover))
        .filter(|(coord, _)| **coord != starting_position)
        .filter_map(|(pos, _)| {
            let mut guardpos = starting_position;
            let mut guard_direction = Direction::North;

            let mut visitations = HashSet::new();

            loop {
                let nextpos = guardpos + guard_direction.to_coord();
                if visitations.contains(&(nextpos, guard_direction)) {
                    return Some(*pos);
                }
                let nextstate = grid.get(&nextpos)?;
                if nextpos == *pos || matches!(nextstate, GridState::Obstacle) {
                    guard_direction = guard_direction.rotate(RotateDirection::Right);
                    continue;
                }
                guardpos = nextpos;
                visitations.insert((guardpos, guard_direction));
            }
        })
        .unique()
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "6",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1
    }),
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
