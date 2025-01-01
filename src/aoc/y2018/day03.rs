// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use crate::{AoCError, CommonGrid, Coord2D, InfGrid};

struct Claim {
    id: usize,
    top_left: Coord2D,
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+,\d+): (\d+)x(\d+)$").unwrap();
        }

        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("input {s} does not match regex")))?;

        Ok(Self {
            id: matches[1].parse().map_err(AoCError::new_from_parseerror)?,
            top_left: matches[2].parse().map_err(AoCError::new_from_parseerror)?,
            width: matches[3].parse().map_err(AoCError::new_from_parseerror)?,
            height: matches[4].parse().map_err(AoCError::new_from_parseerror)?,
        })
    }
}

impl Claim {
    fn get_coords(&self) -> impl Iterator<Item = Coord2D> + use<'_> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| self.top_left + (x, y).try_into().unwrap())
    }
}

enum GridState {
    Claim,
    Conflict,
}

fn griddle(claims: &[Claim]) -> InfGrid<GridState> {
    let mut grid = InfGrid::new();
    for coord in claims.iter().flat_map(|claim| claim.get_coords()) {
        grid.entry(coord)
            .and_modify(|e| *e = GridState::Conflict)
            .or_insert(GridState::Claim);
    }
    grid
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let claims: Vec<_> = data.map(|line| line.parse()).try_collect()?;
    let grid = griddle(&claims);
    let ret = grid
        .into_iter()
        .filter(|(_, v)| matches!(v, GridState::Conflict))
        .count();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let claims: Vec<_> = data.map(|line| line.parse()).try_collect()?;
    let grid = griddle(&claims);
    let ret = claims
        .into_iter()
        .find(|claim| {
            claim
                .get_coords()
                .all(|coord| matches!(grid.get(&coord), Some(GridState::Claim)))
        })
        .map(|claim| claim.id)
        .unwrap();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2018",
    day: "3",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1
    }),
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
