// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;
use num::Integer;

use crate::AoCError;

#[derive(Debug)]
struct Pass {
    row: u32,
    seat: u32,
}

impl FromStr for Pass {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_min = 0;
        let mut row_max = 127;
        let mut seat_min = 0;
        let mut seat_max = 7;

        log::debug!("Boarding pass {s}");

        for c in s.chars() {
            match c {
                'F' => row_max = row_min + (row_max - row_min).div_floor(&2),
                'B' => row_min = row_min + (row_max - row_min).div_ceil(&2),
                'L' => {
                    seat_max = seat_min + (seat_max - seat_min).div_floor(&2);
                }
                'R' => seat_min = seat_min + (seat_max - seat_min).div_ceil(&2),
                _ => return Err(Self::Err::new_from_char(c)),
            }
            log::debug!("{c} {row_min}/{row_max} {seat_min}/{seat_max}");
        }

        if row_min != row_max {
            return Err(Self::Err::new(format!(
                "Row not finalised! {row_min}/{row_max}"
            )));
        } else if seat_min != seat_max {
            return Err(Self::Err::new(format!(
                "Seat not finalised! {seat_min}/{seat_max}"
            )));
        }

        log::debug!(" Result: Row {row_min} Seat {seat_min}");

        Ok(Self {
            row: row_min,
            seat: seat_min,
        })
    }
}

impl Pass {
    fn seat_id(&self) -> u32 {
        self.row * 8 + self.seat
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let passes: Vec<Pass> = data.map(|line| line.parse()).try_collect()?;

    let ret = passes
        .into_iter()
        .map(|pass| pass.seat_id())
        .max()
        .expect("Must be at least one pass");
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let passes: HashSet<u32> = data
        .map(|line| line.parse())
        .map_ok(|pass: Pass| pass.seat_id())
        .try_collect()?;

    let min_seat = passes.iter().min().unwrap();
    let max_seat = passes.iter().max().unwrap();

    let ret = (*min_seat..*max_seat)
        .find(|id| !passes.contains(id))
        .ok_or(AoCError::new("No missing boarding pass!"))?;
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2020",
    day: "5",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});
