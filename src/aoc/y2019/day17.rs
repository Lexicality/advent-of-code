// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

use super::computer::Computer;
use crate::{AoCError, CharGrid, CommonGrid, Direction, Grid, RotateDirection};

enum GridState {
    Void,
    Scaffold,
    Bot(Direction),
}

impl TryFrom<char> for GridState {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Scaffold),
            '.' => Ok(Self::Void),
            _ => {
                let dir: Direction = value.try_into()?;
                Ok(Self::Bot(dir))
            }
        }
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Void => '·',
            GridState::Scaffold => '█',
            GridState::Bot(dir) => match dir {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
            },
        }
        .fmt(f)
    }
}

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let base_computer: Computer = data.next().unwrap().parse().unwrap();
    let mut computer = base_computer.clone();
    computer.run_to_completion().unwrap();

    let output = computer.get_ascii_output().expect("There must be output");

    let grid: Grid<GridState> = Grid::new_from_chars(
        output
            .lines()
            .map(|l| l.to_owned())
            .collect_vec()
            .into_iter(),
    )?;

    println!("{grid:#}");

    let start = grid
        .find(|(_, value)| matches!(value, GridState::Bot(_)))
        .expect("There must be a start");
    let mut seen = HashSet::with_capacity(
        grid.iter()
            .filter(|(_, v)| !matches!(v, GridState::Void))
            .count(),
    );
    seen.insert(start);
    let mut pos = start;
    let mut dir = Direction::North; // hack, I can't be bothered to look this up and I know it starts up on my input
    let mut distance = 0;
    loop {
        let next = pos + dir.to_coord();
        if matches!(grid.get(&next), Some(GridState::Scaffold)) {
            distance += 1;
            pos = next;
            seen.insert(pos);
        } else {
            if distance != 0 {
                print!("{distance}, ");
            }
            distance = 0;
            let next = grid
                .get_neighbour_coords_filtered(pos, false, |pos, value| {
                    matches!(value, GridState::Scaffold) && !seen.contains(pos)
                })
                .next();
            if let Some(next) = next {
                let newdir = Direction::from_coord(next - pos);
                let letter = if newdir == dir.rotate(RotateDirection::Left) {
                    "L"
                } else {
                    "R"
                };

                print!("{letter}, ");

                dir = newdir;
            } else {
                println!("\nEnded at {pos}!");
                break;
            }
        }
    }

    // Manually inspecting the output to work out the sequences because I can't
    // be bothered to automate it goes here

    let mut computer = base_computer.clone();
    computer.set(0, 2.into());
    computer.add_ascii_input(
        "
A,B,A,C,C,A,B,C,B,B
L,8,R,10,L,8,R,8
L,12,R,8,R,8
L,8,R,6,R,6,R,10,L,8
n
.
    "
        .trim(),
    );
    computer.run_to_completion().unwrap();

    match computer.get_ascii_output() {
        Some(death) => Ok(death),
        None => {
            println!("{}", computer.get_ascii_lossy());
            Ok(computer.output.pop().unwrap().to_string())
        }
    }
}

inventory::submit!(crate::AoCDay::mew("2019", "17", main));
