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
    pub mod data;
    pub mod direction;
    pub mod error;
    pub mod grid;
    pub mod infgrid;
    pub mod input_partitioner;
}
pub mod symbols;

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
pub use crate::utils::data::AoCData;
pub use crate::utils::direction::Direction;
pub use crate::utils::direction::RotateDirection;
pub use crate::utils::error::AoCError;
pub use crate::utils::error::AoCResult;
pub use crate::utils::grid::Grid;
pub use crate::utils::infgrid::InfGrid;
pub use crate::utils::input_partitioner::InputPartitioner;

pub type DataIter<'a> = &'a mut dyn Iterator<Item = String>;
pub type DataIn = utils::data::AoCDataIterator;
pub type AoCDayFn = fn(DataIn) -> AoCResult<String>;

pub struct AoCPart {
    pub main: AoCDayFn,
    pub example: AoCDayFn,
}

pub struct AoCDay {
    pub year: &'static str,
    pub day: &'static str,
    pub part_1: AoCPart,
    pub part_2: Option<AoCPart>,
}

inventory::collect!(AoCDay);

pub fn multi_line_example(data: DataIn, main: AoCDayFn) -> AoCResult<String> {
    for line in data {
        println!("Example: {line}");
        let res = main(AoCData::new_from_line(line).into_iter())?;
        println!("Result: {res}\n===");
    }
    Ok("".to_owned())
}

pub fn partitioned_example(data: DataIn, main: AoCDayFn) -> AoCResult<String> {
    for (i, lines) in data.partition().enumerate() {
        println!("Example #{i}");
        let res = main(lines.into_iter())?;
        println!("Result: {res}\n===");
    }
    Ok("".to_owned())
}

pub fn partition_input(data: DataIn) -> (DataIn, DataIn) {
    let mut partitioner = data.partition();

    (
        partitioner
            .next()
            .expect("There must be data in the input!")
            .into_iter(),
        partitioner.next().unwrap_or_default().into_iter(),
    )
}
