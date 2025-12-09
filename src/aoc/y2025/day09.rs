// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use num::Integer;

use crate::{AoCError, BigCoord2D, CommonGrid, Coordinate, Coordinate2D, symbols};

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

/// Adapted from http://web.archive.org/web/20080812141848/http://local.wasp.uwa.edu.au/~pbourke/geometry/insidepoly/
fn inside_polygon(point: BigCoord2D, polygon: &[BigCoord2D]) -> bool {
    polygon
        .iter()
        .tuple_windows()
        .filter(|(poly_a, poly_b)| {
            let poly_min = poly_a.get_min(poly_b);
            let poly_max = poly_a.get_max(poly_b);
            if point.y <= poly_min.y {
                // why does this only check y coords? who knows.
                return false;
            } else if point.x > poly_max.x || point.y > poly_max.y {
                return false;
            } else if poly_a.y == poly_b.y {
                // ????
                return false;
            }
            // mystery statement
            let xinters =
                (point.y - poly_a.y) * (poly_b.x - poly_a.x) / (poly_b.y - poly_a.y) + poly_a.x;
            poly_a.x == poly_b.x || point.x <= xinters
        })
        .count()
        .is_odd()
}

fn cached_inside_polygon(
    point: BigCoord2D,
    polygon: &[BigCoord2D],
    prechecked: &mut HashMap<BigCoord2D, bool>,
) -> bool {
    if let Some(result) = prechecked.get(&point) {
        return *result;
    }
    let result = inside_polygon(point, polygon);
    log::trace!(
        "Magic function says {point} is {} poly",
        if result { "inside" } else { "outside" }
    );
    prechecked.insert(point, result);
    result
}

fn rectangulate(a: &BigCoord2D, b: &BigCoord2D) -> impl Iterator<Item = BigCoord2D> {
    log::trace!("wtf wtf {a} {b}");
    // I'm moderately sure I've written this code before but I don't know where it is
    (a.x..=b.x)
        .cartesian_product(a.y..=b.y)
        .map(BigCoord2D::from_tuple)
        .inspect(|c| log::trace!("wtf {c}"))
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let coords = {
        let mut coords: Vec<BigCoord2D> = data.map(|line| line.parse()).try_collect()?;
        // Make it into a closed polygon
        coords.push(coords[0]);
        coords
    };

    let mut prechecked: HashMap<BigCoord2D, bool> = coords
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
        .map(|c| (c, true))
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
            prechecked.keys().map(|a| (*a, GridState::Green)).collect();

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
        .find(|(_, a, b)| {
            rectangulate(a, b).all(|point| cached_inside_polygon(point, &coords, &mut prechecked))
        })
        .map(|(dist, _, _)| dist)
        .ok_or(AoCError::new("No valid rectangles?"))?;

    // stupid test grid v2
    log::debug!("postchecked:\n{}", {
        use aoc_macros::VoidState;

        use crate::SparseGrid;

        #[derive(Debug, VoidState)]
        enum GridState {
            #[void]
            Void,
            Red,
            Green,
            White,
        }

        impl Display for GridState {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Void => symbols::VOID,
                    Self::Red => symbols::BLOCK,
                    Self::Green => symbols::SHADE_DARK,
                    Self::White => symbols::SHADE_LIGHT,
                }
                .fmt(f)
            }
        }

        let mut grid: SparseGrid<GridState, BigCoord2D> = prechecked
            .iter()
            .map(|(coord, state)| {
                (
                    *coord,
                    if *state {
                        GridState::Green
                    } else {
                        GridState::White
                    },
                )
            })
            .collect();

        coords.iter().for_each(|c| {
            grid.set(*c, GridState::Red);
        });

        grid
    });

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
