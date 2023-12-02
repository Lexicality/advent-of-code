use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

use crate::{AoCError, Coord2D, Direction, InfGrid};

struct Instruction {
    direction: Direction,
    amount: i32,
}

impl FromStr for Instruction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = s.split_at(1);
        Ok(Instruction {
            direction: match dir {
                "U" => Direction::North,
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => return Err(AoCError::new(format!("Unknown direction {dir}"))),
            },
            amount: amt
                .parse::<u32>()
                .map_err(|e| AoCError::new_with_cause("Invalid amount", e))?
                as i32,
        })
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum GridState {
    #[default]
    Void,
    First,
    Second,
    Both,
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Void => ".",
                Self::First => "F",
                Self::Second => "S",
                Self::Both => "X",
            }
        )
    }
}

fn wire_it_up(
    grid: &mut InfGrid<GridState>,
    instructions: &str,
    state: GridState,
) -> Result<(), AoCError> {
    let instructions: Vec<Instruction> = instructions
        .split(',')
        .map(|i| i.parse())
        .collect::<Result<_, _>>()?;
    let mut pos = Coord2D { x: 0, y: 0 };
    for instruction in instructions {
        let coord: Coord2D = instruction.direction.into();
        for _ in 0..instruction.amount {
            pos = pos + coord;
            let result = grid.get_or_set(&pos, state);
            if *result != state {
                grid.set(pos, GridState::Both);
            }
        }
    }
    Ok(())
}

pub fn main(data: crate::DataIn) -> String {
    let mut ret = 0;
    while let Some(first) = data.next() {
        let second = data.next().expect("Lines should be paired");
        let mut grid = InfGrid::<GridState>::new();
        wire_it_up(&mut grid, &first, GridState::First).unwrap();
        wire_it_up(&mut grid, &second, GridState::Second).unwrap();
        let crossover = grid
            .iter()
            .filter_map(|(pos, state)| (*state == GridState::Both).then_some(pos).copied())
            .sorted_by_key(|pos| pos.len_manhatten())
            .next()
            .expect("Must have at least one crossover");
        ret = crossover.len_manhatten();
        println!("Crosses at {crossover} which is {ret} away");
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "3",
    func: main,
});
