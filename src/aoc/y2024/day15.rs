// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use itertools::Itertools;

use crate::{partition_input, symbols, AoCError, CharGrid, CommonGrid, Direction, Grid};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum GridState {
    Empty,
    Wall,
    Robot,
    Crate,
}

impl TryFrom<char> for GridState {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '@' => Self::Robot,
            'O' => Self::Crate,
            _ => return Err(AoCError::new_from_char(value)),
        })
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Empty => symbols::VOID,
            GridState::Wall => symbols::BLOCK,
            GridState::Robot => symbols::ROBOT,
            GridState::Crate => symbols::BOX,
        }
        .fmt(f)
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let (griddata, instructions) = partition_input(data);
    let mut grid: Grid<GridState> = Grid::new_from_chars(griddata)?;
    let instructions: Vec<Direction> = instructions
        .flat_map(|line| -> Vec<_> { line.chars().collect() })
        .map(char::try_into)
        .try_collect()?;

    println!("{grid}");

    let mut robot_pos = grid
        .find(|(_, v)| matches!(v, GridState::Robot))
        .expect("Robot must exist");

    for instruction in instructions {
        // println!("Robot should move {instruction}");
        let march_dir = instruction.to_coord();
        let mut target_pos = robot_pos;
        let mut stack = vec![GridState::Robot];
        loop {
            target_pos += march_dir;
            match grid.get(&target_pos).unwrap() {
                GridState::Wall => {
                    break;
                }
                GridState::Empty => {
                    *grid.get_mut(&robot_pos).unwrap() = GridState::Empty;
                    for v in stack.into_iter().rev() {
                        *grid.get_mut(&target_pos).unwrap() = v;
                        if matches!(v, GridState::Robot) {
                            robot_pos = target_pos;
                            break;
                        }
                        target_pos -= march_dir;
                    }
                    break;
                }
                GridState::Crate => {
                    stack.push(GridState::Crate);
                }
                GridState::Robot => unreachable!(),
            }
        }
        // println!("{grid}");
    }

    println!("{grid}");

    let ret: i32 = grid
        .into_iter()
        .filter_map(|(coord, v)| match v {
            GridState::Crate => Some(coord.x + coord.y * 100),
            _ => None,
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "15",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None,
});
