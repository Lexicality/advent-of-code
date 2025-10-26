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

use crate::{AoCError, Coordinate, InfGrid, utils::bigcoord2d::BigCoord2D};

enum Space {
    Void,
    Galaxy,
}

impl TryFrom<char> for Space {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Void),
            '#' => Ok(Self::Galaxy),
            _ => Err(AoCError::new(format!("Unknown character {value}"))),
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => '.',
            Self::Galaxy => '#',
        }
        .fmt(f)
    }
}

fn main(data: crate::DataIn, expansion_factor: usize) -> crate::AoCResult<String> {
    let mut starscape: InfGrid<Space, BigCoord2D> = data
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| ((x, y).try_into().unwrap(), Space::Galaxy))
                .collect_vec()
        })
        .collect();

    // println!("{starscape:#}");

    let mut columns: HashSet<i64> = HashSet::new();
    let mut rows: HashSet<i64> = HashSet::new();
    for coord in starscape.grid.keys() {
        columns.insert(coord.x);
        rows.insert(coord.y);
    }
    let expansion_cols = (starscape.min.x..=starscape.max.x)
        .filter(|x| !columns.contains(x))
        .collect_vec();
    let expansion_rows = (starscape.min.y..=starscape.max.y)
        .filter(|y| !rows.contains(y))
        .collect_vec();

    starscape = starscape
        .into_iter()
        .map(|(mut coord, value)| {
            let expand_x =
                expansion_cols.iter().filter(|x| **x < coord.x).count() * expansion_factor;
            let expand_y =
                expansion_rows.iter().filter(|y| **y < coord.y).count() * expansion_factor;
            coord += (expand_x, expand_y).try_into().unwrap();
            (coord, value)
        })
        .collect();

    // println!("{starscape:#}");

    let ret: i64 = starscape
        .grid
        .keys()
        .combinations(2)
        .map(|ab| ab[0].distance(ab[1]))
        .sum();

    Ok(ret.to_string())
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    main(data, 1)
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    main(data, 999_999)
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "11",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
