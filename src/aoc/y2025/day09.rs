// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCError, BigCoord2D, CommonGrid, Coordinate, symbols};

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

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let coords = {
        let mut coords: Vec<BigCoord2D> = data.map(|line| line.parse()).try_collect()?;
        // Make it into a closed polygon
        coords.push(coords[0]);
        coords
    };

    let lines: Vec<BigCoord2D> = coords
        .iter()
        .tuple_windows()
        .flat_map(|(a, b)| {
            let (a, b) = (a.get_min(b), a.get_max(b));
            log::trace!("ye {a} {b}");
            if a.y == b.y {
                let y = a.y;
                (a.x..=b.x).map(|x| BigCoord2D { x, y }).collect_vec()
            } else {
                let x = a.x;
                (a.y..=b.y).map(|y| BigCoord2D { x, y }).collect_vec()
            }
        })
        .collect();

    // stupid test grid
    log::debug!("prechecked:\n{}", {
        use aoc_macros::VoidState;

        use crate::SparseGrid;

        #[derive(Debug, VoidState)]
        enum GridState {
            #[void]
            Void,
            Red,
            Green,
        }

        impl Display for GridState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Void => symbols::SHADE_LIGHT,
                    Self::Red => symbols::BLOCK,
                    Self::Green => symbols::SHADE_MID,
                }
                .fmt(f)
            }
        }

        let mut grid: SparseGrid<GridState, BigCoord2D> =
            lines.iter().map(|a| (*a, GridState::Green)).collect();

        coords.iter().for_each(|c| {
            grid.set(*c, GridState::Red);
        });

        grid
    });

    let ret = coords
        .iter()
        .array_combinations()
        .map(|[a, b]| {
            let min = a.get_min(b);
            let max = a.get_max(b);
            ((1 + max.x - min.x) * (1 + max.y - min.y), min, max)
        })
        .sorted_unstable()
        .rev()
        .inspect(|(dist, a, b)| {
            log::debug!("Square of area {dist} between {a} and {b}");
        })
        .filter(|(_, min, max)| {
            !coords
                .iter()
                .any(|c| c.x > min.x && c.y > min.y && c.x < max.x && c.y < max.y)
        })
        .find(|(_, min, max)| {
            !lines
                .iter()
                .any(|c| c.x > min.x && c.y > min.y && c.x < max.x && c.y < max.y)
        })
        .map(|(dist, _, _)| dist)
        .ok_or(AoCError::new("No valid rectangles?"))?;

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "9",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2,
    })
});
