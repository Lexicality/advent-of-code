// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;
use std::fmt::Display;

use itertools::Itertools;

use crate::symbols;
use crate::AoCError;
use crate::CharGrid;
use crate::CommonGrid;
use crate::Coord2D;
use crate::Grid;

enum GridState {
    Empty,
    Antenna(char),
    Antinode,
}

impl TryFrom<char> for GridState {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            c if c.is_ascii_alphanumeric() => Self::Antenna(c),
            _ => return Err(AoCError::new_from_char(value)),
        })
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Empty => symbols::VOID,
            GridState::Antenna(c) => *c,
            GridState::Antinode => '#',
        }
        .fmt(f)
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid: Grid<GridState> = Grid::new_from_chars(data)?;
    println!("{grid:#}");
    let mut antinodes: HashMap<char, Vec<Coord2D>> = grid
        .iter()
        .filter_map(|(_, s)| match s {
            GridState::Antenna(c) => Some(c),
            _ => None,
        })
        .unique()
        .map(|c| (*c, vec![]))
        .collect();

    let tmp = grid
        .iter()
        .filter_map(|(coord, s)| match s {
            GridState::Antenna(c) => Some((coord, c)),
            _ => None,
        })
        .sorted_by_cached_key(|(_, c)| *c)
        .chunk_by(|(_, c)| *c);

    for (signal, coords) in tmp.into_iter() {
        let antinodes = antinodes
            .get_mut(signal)
            .expect("we already prefilled this");
        // for coords in coords.map(|(coord, _)| coord).permutations(2) {
        //     let a = coords[0];
        //     let b = coords[1];
        let coords: Vec<_> = coords.map(|(coord, _)| coord).collect();
        // silly
        for coord in coords.iter() {
            antinodes.push(**coord);
        }
        for (a, b) in coords.into_iter().tuple_combinations() {
            let step1 = a - b;
            let mut first = *a + step1;
            while grid.check_coord(&first) {
                antinodes.push(first);
                first += step1;
            }
            let step2 = b - a;
            let mut second = *b + step2;
            while grid.check_coord(&second) {
                antinodes.push(second);
                second += step2;
            }
        }
    }

    // decorate the grid
    for coord in antinodes.values().flatten() {
        let current = grid.get_mut(coord).unwrap();
        if matches!(current, GridState::Empty) {
            *current = GridState::Antinode;
        }
    }

    println!("{grid:#}");

    let ret = antinodes
        .into_iter()
        .flat_map(|(_, nodes)| nodes.into_iter())
        .unique()
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "8", main));
