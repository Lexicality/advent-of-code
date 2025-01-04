// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

use itertools::Itertools;

use crate::{CommonGrid, Coord2D, FlatGrid, Grid};

#[derive(Debug, Clone, Copy)]
enum GridState {
    Dot,
    Digit(char),
    Symbol(char),
}

impl From<char> for GridState {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Dot,
            digit @ '0'..='9' => Self::Digit(digit),
            symbol => Self::Symbol(symbol),
        }
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridState::Dot => &'.',
                GridState::Digit(char) | GridState::Symbol(char) => char,
            }
        )
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut data = data.peekable();
    let width = data.peek().unwrap().len().try_into().unwrap();
    let data = data
        .flat_map(|line| line.chars().collect_vec())
        .map(GridState::from);
    let grid = Grid::new_from_iter(data, width);

    let symbols = grid
        .iter()
        .filter(|(_, state)| matches!(state, GridState::Symbol('*')));

    let mut ret = 0;
    let mut seen = HashSet::new();

    for (symbol_coord, _symbol) in symbols {
        let surrounding_numbers = grid
            .get_neighbour_coords(symbol_coord.to_owned(), true)
            .filter_map(|coord| {
                let state = grid.get(&coord).unwrap();
                match state {
                    GridState::Digit(digit) => Some((coord, digit)),
                    _ => None,
                }
            });
        for (digit_coord, digit) in surrounding_numbers {
            if !seen.insert(digit_coord) {
                continue;
            }
            let mut current_number = VecDeque::new();
            current_number.push_front(digit);
            const LEFT: Coord2D = Coord2D { x: -1, y: 0 };
            let mut pos = digit_coord + LEFT;
            while let Some(GridState::Digit(digit)) = grid.get(&pos) {
                assert!(
                    seen.insert(pos),
                    "should not be possible to get somewhere already seen"
                );
                current_number.push_front(digit);
                pos += LEFT;
            }
            const RIGHT: Coord2D = Coord2D { x: 1, y: 0 };
            pos = digit_coord + RIGHT;
            while let Some(GridState::Digit(digit)) = grid.get(&pos) {
                assert!(
                    seen.insert(pos),
                    "should not be possible to get somewhere already seen"
                );
                current_number.push_back(digit);
                pos += RIGHT;
            }
            // I'm sure there's a smarter way of doing this
            let number = current_number
                .iter()
                .join("")
                .parse::<u64>()
                .expect("should be a number");
            // println!("Found number {number}!");
            ret += number;
        }
    }

    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut data = data.peekable();
    let width = data.peek().unwrap().len().try_into().unwrap();
    let data = data
        .flat_map(|line| line.chars().collect_vec())
        .map(GridState::from);
    let grid = Grid::new_from_iter(data, width);

    let symbols = grid
        .iter()
        .filter(|(_, state)| matches!(state, GridState::Symbol('*')));

    let mut ret = 0;

    for (symbol_coord, _symbol) in symbols {
        let surrounding_numbers = grid
            .get_neighbour_coords(symbol_coord.to_owned(), true)
            .filter_map(|coord| {
                let state = grid.get(&coord).unwrap();
                match state {
                    GridState::Digit(digit) => Some((coord, digit)),
                    _ => None,
                }
            });
        let mut seen = HashSet::new();
        let mut numbers = Vec::with_capacity(2);
        for (digit_coord, digit) in surrounding_numbers {
            if !seen.insert(digit_coord) {
                continue;
            }
            let mut current_number = VecDeque::new();
            current_number.push_front(digit);
            const LEFT: Coord2D = Coord2D { x: -1, y: 0 };
            let mut pos = digit_coord + LEFT;
            while let Some(GridState::Digit(digit)) = grid.get(&pos) {
                assert!(
                    seen.insert(pos),
                    "should not be possible to get somewhere already seen"
                );
                current_number.push_front(digit);
                pos += LEFT;
            }
            const RIGHT: Coord2D = Coord2D { x: 1, y: 0 };
            pos = digit_coord + RIGHT;
            while let Some(GridState::Digit(digit)) = grid.get(&pos) {
                assert!(
                    seen.insert(pos),
                    "should not be possible to get somewhere already seen"
                );
                current_number.push_back(digit);
                pos += RIGHT;
            }
            // I'm sure there's a smarter way of doing this
            let number = current_number
                .iter()
                .join("")
                .parse::<u64>()
                .expect("should be a number");
            numbers.push(number);
            // println!("Found number {number}!");
            // ret += number;
        }
        if numbers.len() == 2 {
            ret += numbers[0] * numbers[1];
        }
    }

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
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
