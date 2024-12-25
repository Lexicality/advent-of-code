// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

pub mod aoc;
pub mod utils {
    pub mod astar;
    pub mod bigcoord2d;
    pub mod bigcoord3d;
    pub mod commongrid;
    pub mod coord;
    pub mod coord2d;
    pub mod coord3d;
    pub mod direction;
    pub mod error;
    pub mod grid;
    pub mod infgrid;
    pub mod input_partitioner;
}
pub mod symbols;

use std::iter::IntoIterator;

pub use crate::utils::bigcoord2d::BigCoord2D;
pub use crate::utils::bigcoord3d::BigCoord3D;
pub use crate::utils::commongrid::CharGrid;
pub use crate::utils::commongrid::CommonGrid;
pub use crate::utils::commongrid::DisplayGrid;
pub use crate::utils::commongrid::FlatGrid;
pub use crate::utils::coord::Coordinate;
pub use crate::utils::coord::Coordinate2D;
pub use crate::utils::coord2d::Coord2D;
pub use crate::utils::coord3d::Coord3D;
pub use crate::utils::direction::Direction;
pub use crate::utils::direction::RotateDirection;
pub use crate::utils::error::AoCError;
pub use crate::utils::error::AoCResult;
pub use crate::utils::grid::Grid;
pub use crate::utils::infgrid::InfGrid;
pub use crate::utils::input_partitioner::InputPartitioner;

pub type DataIter<'a> = &'a mut dyn Iterator<Item = String>;
pub type DataIn = <Vec<String> as IntoIterator>::IntoIter;
pub type AoCDayFn = fn(DataIn) -> AoCResult<String>;

pub struct AoCDay {
    pub year: &'static str,
    pub day: &'static str,
    pub func: AoCDayFn,
    pub example_func: Option<AoCDayFn>,
}

const ZERO: u8 = "0".as_bytes()[0];

impl AoCDay {
    const fn mew(year: &'static str, day: &'static str, main: AoCDayFn) -> AoCDay {
        assert!(day.is_ascii(), "Day must be an ASCII number");
        if day.len() == 1 && day.as_bytes()[0] == ZERO {
            panic!("Zero is not a valid day");
        }

        Self {
            year,
            day,
            func: main,
            example_func: None,
        }
    }

    const fn mew_with_example(
        year: &'static str,
        day: &'static str,
        main: AoCDayFn,
        example: AoCDayFn,
    ) -> AoCDay {
        assert!(day.is_ascii(), "Day must be an ASCII number");
        if day.len() == 1 && day.as_bytes()[0] == ZERO {
            panic!("Zero is not a valid day");
        }

        Self {
            year,
            day,
            func: main,
            example_func: Some(example),
        }
    }
}

inventory::collect!(AoCDay);

pub fn multi_line_example(data: DataIn, main: AoCDayFn) -> AoCResult<String> {
    for line in data {
        println!("Example: {line}");
        let res = main(vec![line.to_owned()].into_iter())?;
        println!("Result: {res}\n===");
    }
    Ok("".to_owned())
}

pub fn partitioned_example(data: DataIn, main: AoCDayFn) -> AoCResult<String> {
    for (i, lines) in InputPartitioner::new(data, |line| !line.is_empty()).enumerate() {
        println!("Example #{i}");
        let res = main(lines.into_iter())?;
        println!("Result: {res}\n===");
    }
    Ok("".to_owned())
}

pub fn partition_input(data: DataIn) -> (DataIn, DataIn) {
    let mut partitioner = InputPartitioner::new(data, |line| !line.is_empty());

    let a = partitioner
        .next()
        .expect("There must be data in the input!");
    let b = partitioner.next().unwrap_or_default();
    (a.into_iter(), b.into_iter())
}
