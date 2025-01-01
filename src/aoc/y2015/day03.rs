// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashSet;

use itertools::Itertools;

use crate::{Coord2D, Direction};

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();

    let dirs: Vec<Direction> = line.chars().map(|c| c.try_into()).try_collect()?;
    let mut seen = HashSet::with_capacity(dirs.len());
    let start: Coord2D = Default::default();
    seen.insert(start);
    let _pos = dirs.into_iter().fold(start, |mut pos, dir| {
        pos += dir.to_coord();
        seen.insert(pos);
        pos
    });

    Ok(seen.len().to_string())
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();

    let dirs: Vec<Direction> = line.chars().map(|c| c.try_into()).try_collect()?;
    let mut seen = HashSet::with_capacity(dirs.len());
    let start: Coord2D = Default::default();
    seen.insert(start);
    dirs.iter().step_by(2).fold(start, |mut pos, dir| {
        pos += dir.to_coord();
        seen.insert(pos);
        pos
    });

    dirs.into_iter()
        .skip(1)
        .step_by(2)
        .fold(start, |mut pos, dir| {
            pos += dir.to_coord();
            seen.insert(pos);
            pos
        });

    Ok(seen.len().to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "3",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: |data| crate::multi_line_example(data, part_1),
    }),
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::multi_line_example(data, part_2),
    }),
});
