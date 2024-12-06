use std::fmt::Display;

use crate::{utils::direction::RotateDirection, AoCError, CharGrid, CommonGrid, Direction, Grid};

#[derive(Debug, Clone, Copy)]
enum GridState {
    Empty,
    Obstacle,
    Guard,
    Visited,
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
            GridState::Empty => '.',
            GridState::Obstacle => '#',
            GridState::Guard => '^',
            GridState::Visited => 'X',
        }
        .fmt(f)
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::new_from_chars(data)?;
    let mut guardpos = grid
        .find(|(_, v)| matches!(v, GridState::Guard))
        .expect("Guard must exist");
    let starting_position = guardpos;
    println!("{grid:#}");
    grid.set(guardpos, GridState::Visited);
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
        grid.set(guardpos, GridState::Visited);
    }

    let ret = grid
        .into_iter()
        .filter(|(_, v)| matches!(v, GridState::Visited))
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "6", main));
