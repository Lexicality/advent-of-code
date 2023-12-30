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
}

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
pub use crate::utils::error::AoCError;
pub use crate::utils::error::AoCResult;
pub use crate::utils::grid::Grid;
pub use crate::utils::infgrid::InfGrid;

pub type DataIn<'a> = &'a mut dyn Iterator<Item = String>;
pub type AoCDayFn = fn(DataIn) -> AoCResult<String>;

pub struct AoCDay {
    pub year: &'static str,
    pub day: &'static str,
    pub func: AoCDayFn,
    pub example_func: Option<AoCDayFn>,
}

inventory::collect!(AoCDay);

pub fn multi_line_example(data: DataIn, main: AoCDayFn) -> AoCResult<String> {
    for line in data {
        println!("Example: {line}");
        let res = main(&mut vec![line].into_iter())?;
        println!("Result: {res}\n===");
    }
    Ok("".to_owned())
}
