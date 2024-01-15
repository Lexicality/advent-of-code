use std::fmt::Display;

use itertools::Itertools;

use super::computer::Computer;
use crate::{AoCError, CharGrid, Direction, Grid};

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
            GridState::Void => '.',
            GridState::Scaffold => '#',
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
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
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

    let mut ret = 0;
    // extremely simplistic intersection detection
    for coord in grid
        .iter()
        .filter(|(_, value)| matches!(value, GridState::Scaffold))
        .map(|(pos, _)| pos)
    {
        if grid
            .get_neighbours_filtered(*coord, false, |_, v| matches!(v, GridState::Scaffold))
            .count()
            == 4
        {
            ret += coord.x * coord.y;
        }
    }

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2019", "17", main));
