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

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
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

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    crate::multi_line_example(data, main)
}

inventory::submit!(crate::AoCDay::mew_with_example(
    "2015",
    "3",
    main,
    main_example
));
