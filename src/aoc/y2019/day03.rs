// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

use crate::{AoCError, AoCResult, CommonGrid, Coord2D, Coordinate, Direction, InfGrid};

struct Instruction {
    direction: Direction,
    amount: i32,
}

impl FromStr for Instruction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = s.split_at(1);
        Ok(Instruction {
            direction: dir.parse()?,
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
            pos += coord;
            let result = grid.get_or_set(&pos, state);
            if *result != state {
                grid.set(pos, GridState::Both);
            }
        }
    }
    Ok(())
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
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
    Ok(ret.to_string())
}

fn steppinator(grid: &InfGrid<GridState>, first: &str, second: &str) -> AoCResult<i32> {
    let mut retdata = InfGrid::<i32>::new();
    for instructions in [first, second].iter() {
        let instructions: Vec<Instruction> = instructions
            .split(',')
            .map(|i| i.parse())
            .collect::<Result<_, _>>()?;
        let mut pos = Coord2D { x: 0, y: 0 };
        let mut steps = 0;
        for instruction in instructions {
            let coord: Coord2D = instruction.direction.into();
            for _ in 0..instruction.amount {
                pos += coord;
                steps += 1;
                match grid.get(&pos) {
                    Some(GridState::Both) => {
                        let amt = *retdata.get_or_set_default(&pos);
                        retdata.set(pos, amt + steps);
                    }
                    Some(_) => (),
                    None => return Err(AoCError::new(format!("path broke at {pos}"))),
                }
            }
        }
    }
    retdata
        .iter()
        .map(|(_, l)| l)
        .sorted()
        .copied()
        .next()
        .ok_or(AoCError::new("no crossovers?"))
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
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
        let best = steppinator(&grid, &first, &second).unwrap();
        ret = best;
        println!("best {best}");
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "3",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
