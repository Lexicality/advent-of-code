// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use crate::{AoCResult, CommonGrid, Coord2D, Direction, InfGrid, RotateDirection};

use super::computer::{Computer, RunState};

#[derive(Debug, Default, Clone, Copy)]
enum Colour {
    #[default]
    Black = 0,
    White = 1,
}

impl Colour {
    const fn to_input(self) -> i64 {
        self as i64
    }

    const fn from_value(value: i64) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::White,
            _ => unreachable!(),
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Black => ' ',
                Self::White => '#',
            }
        )
    }
}

struct Robot {
    cpu: Computer,
    direction: Direction,
    pos: Coord2D,
}

impl Robot {
    fn new(program: String) -> AoCResult<Self> {
        Ok(Robot {
            cpu: program.parse()?,
            direction: Direction::North,
            pos: Default::default(),
        })
    }

    fn drive(&mut self, current_square: Colour) -> Option<(Coord2D, Colour)> {
        self.cpu.input.push_back(current_square.to_input());
        let res = self.cpu.run().unwrap();
        if matches!(res, RunState::Finished) || self.cpu.output.is_empty() {
            return None;
        } else if self.cpu.output.len() != 2 {
            panic!("unexpected cpu output {:?}", self.cpu.output);
        }
        let ret = (self.pos, Colour::from_value(self.cpu.output[0]));
        let rotate_dir = match self.cpu.output[1] {
            0 => RotateDirection::Left,
            1 => RotateDirection::Right,
            _ => unreachable!("mystery rotation!"),
        };
        self.cpu.clear_output();
        self.direction = self.direction.rotate(rotate_dir);
        self.pos += self.direction.to_coord();
        Some(ret)
    }
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut robot = Robot::new(data.next().unwrap()).unwrap();
    let mut hull: InfGrid<Colour> = InfGrid::new();

    let mut current_square = Colour::Black;
    while let Some((pos, colour)) = robot.drive(current_square) {
        hull.set(pos, colour);
        current_square = hull.get(&robot.pos).copied().unwrap_or_default();
    }
    println!("{hull:-}\n");

    Ok(hull.grid.len().to_string())
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut robot = Robot::new(data.next().unwrap()).unwrap();
    let mut hull: InfGrid<Colour> = InfGrid::new();

    let mut current_square = Colour::White;
    while let Some((pos, colour)) = robot.drive(current_square) {
        hull.set(pos, colour);
        current_square = hull.get(&robot.pos).copied().unwrap_or_default();
    }
    println!("{hull:-}\n");

    Ok(hull.grid.len().to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "11",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1
    }),
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
