// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

use crate::{AoCError, BigCoord2D, Coordinate};

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let coords: Vec<BigCoord2D> = data.map(|line| line.parse()).try_collect()?;

    let ret = coords
        .into_iter()
        .array_combinations()
        .map(|[a, b]| {
            let min = a.get_min(&b);
            let max = a.get_max(&b);
            ((1 + max.x - min.x) * (1 + max.y - min.y), a, b)
        })
        .map(|(dist, a, b)| {
            log::debug!("Square of area {dist} between {a} and {b}");
            dist
        })
        .sorted_unstable()
        .next_back()
        .ok_or(AoCError::new("Not enough coords??"))?;

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "9",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None
});
