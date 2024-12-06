use std::collections::HashSet;
use std::fmt::Display;

use itertools::Itertools;

use crate::symbols;
use crate::utils::direction::RotateDirection;
use crate::{AoCError, CharGrid, CommonGrid, Direction, Grid};

#[derive(Debug, Clone, Copy)]
enum GridState {
    Empty,
    Obstacle,
    Guard,
    Visited(Direction),
    Crossover,
    NewBlock,
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
            GridState::NewBlock => 'O'.fmt(f),
        }
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::new_from_chars(data)?;
    let mut guardpos = grid
        .find(|(_, v)| matches!(v, GridState::Guard))
        .expect("Guard must exist");
    let starting_position = guardpos;
    let mut guard_direction = Direction::North;
    grid.set(guardpos, GridState::Visited(guard_direction));
    let starting_grid = grid.clone();
    let mut visitations = HashSet::new();
    let mut unacceptable = HashSet::new();
    loop {
        let nextpos = guardpos + guard_direction.to_coord();
        let Some(nextstate) = grid.get(&nextpos) else {
            unacceptable.insert((guardpos, guard_direction));
            break;
        };
        match nextstate {
            GridState::Obstacle => {
                // Don't try inserting an obstacle where one can't fit
                unacceptable.insert((guardpos, guard_direction));
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
        visitations.insert((guardpos, guard_direction));
    }

    // println!("{grid:#}");

    let ret = visitations
        .iter()
        .filter(|(coord, dir)| (*coord + dir.to_coord()) != starting_position)
        .filter(|step| !unacceptable.contains(step))
        .filter_map(|(pos, direction)| {
            let mut guardpos = starting_position;
            let mut guard_direction = Direction::North;

            let mut visitations = HashSet::new();

            let mut grid = starting_grid.clone();
            let collide_pos = *pos + direction.to_coord();
            grid.set(collide_pos, GridState::NewBlock);
            loop {
                let nextpos = guardpos + guard_direction.to_coord();
                if visitations.contains(&(nextpos, guard_direction)) {
                    // println!("{grid:#}");
                    return Some(collide_pos);
                }
                let nextstate = grid.get(&nextpos)?;
                match nextstate {
                    GridState::Obstacle | GridState::NewBlock => {
                        guard_direction = guard_direction.rotate(RotateDirection::Right);
                        continue;
                    }
                    // GridState::Empty => {
                    //     grid.set(nextpos, GridState::Visited(guard_direction));
                    // }
                    // GridState::Visited(_) => {
                    //     grid.set(nextpos, GridState::Crossover);
                    // }
                    _ => (),
                }
                guardpos = nextpos;
                visitations.insert((guardpos, guard_direction));
            }
        })
        .unique()
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "6", main));
