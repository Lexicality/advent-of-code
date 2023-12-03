use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

use itertools::Itertools;

use crate::{Coord2D, Grid};

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

pub fn main(data: crate::DataIn) -> String {
    let mut data = data.peekable();
    let width = data.peek().unwrap().len().try_into().unwrap();
    let data = data
        .flat_map(|line| line.chars().collect_vec())
        .map(GridState::from);
    let grid = Grid::new_from_data(data, width);

    let symbols = grid
        .iter()
        .filter(|(_, state)| matches!(state, GridState::Symbol(_)));

    let mut ret = 0;
    let mut seen = HashSet::new();

    for (symbol_coord, _symbol) in symbols {
        let surrounding_numbers = grid
            .get_neighbours(symbol_coord.to_owned(), true)
            .filter_map(|coord| {
                let state = grid.get(&coord).unwrap();
                match state {
                    GridState::Digit(digit) => Some((coord, digit)),
                    _ => None,
                }
            });
        // println!("Looking at {symbol} at {symbol_coord}");
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
                pos = pos + LEFT;
            }
            const RIGHT: Coord2D = Coord2D { x: 1, y: 0 };
            pos = digit_coord + RIGHT;
            while let Some(GridState::Digit(digit)) = grid.get(&pos) {
                assert!(
                    seen.insert(pos),
                    "should not be possible to get somewhere already seen"
                );
                current_number.push_back(digit);
                pos = pos + RIGHT;
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

    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "3",
    func: main,
});
