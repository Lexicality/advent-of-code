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

use crate::{AoCError, AoCResult, CharGrid, CommonGrid, Coord2D, Coordinate2D, InfGrid};

#[derive(Debug, Clone, Copy)]
enum PixelState {
    Dark,
    Light,
}

impl PixelState {
    fn to_bit(self) -> usize {
        match self {
            Self::Dark => 0,
            Self::Light => 1,
        }
    }
}

impl TryFrom<char> for PixelState {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Light),
            '.' => Ok(Self::Dark),
            _ => Err(AoCError::new(format!("Unknown pixel value '{value}'"))),
        }
    }
}

impl Display for PixelState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dark => '.',
            Self::Light => '#',
        }
        .fmt(f)
    }
}

#[derive(Debug)]
struct Enhancer(Vec<PixelState>);

impl Enhancer {
    fn parse(line: String) -> AoCResult<Self> {
        Ok(Self(line.chars().map(PixelState::try_from).try_collect()?))
    }

    fn enhance<'a>(&self, values: impl Iterator<Item = &'a PixelState>) -> PixelState {
        let mut val = 0;
        for value in values {
            // print!("{}", value);
            val <<= 1;
            val += value.to_bit();
        }
        self.0.get(val).copied().expect("it should be ok")
    }

    fn get_infinity(&self, current_inf: PixelState) -> PixelState {
        match current_inf {
            PixelState::Dark => self.0.first().copied().unwrap(),
            PixelState::Light => self.0.last().copied().unwrap(),
        }
    }
}

pub fn main(mut data: crate::DataIn, iterations: usize) -> crate::AoCResult<String> {
    let enhancer = Enhancer::parse(data.next().unwrap())?;

    data.next().unwrap();

    let mut grid = InfGrid::new_from_chars(data)?;

    let mut infstate = PixelState::Dark;

    for _ in 0..iterations {
        let min: Coord2D = grid.min_key();
        let max = grid.max_key();

        grid = ((min.y - 2)..=(max.y + 2))
            .cartesian_product((min.x - 2)..=(max.x + 2))
            .map(|(y, x)| (x, y))
            .filter_map(Coord2D::try_from_tuple)
            .map(|coord| {
                (
                    coord,
                    enhancer.enhance(
                        grid.get_neighbours(coord, true, true)
                            .map(|(_, v)| v.unwrap_or(&infstate)),
                    ),
                )
            })
            .collect();
        infstate = enhancer.get_infinity(infstate);
    }

    let ret = grid
        .into_iter()
        .map(|(_, value)| value.to_bit())
        .reduce(usize::saturating_add)
        .unwrap();
    Ok(ret.to_string())
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    main(data, 2)
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    main(data, 50)
}

inventory::submit!(crate::AoCDay {
    year: "2021",
    day: "20",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
