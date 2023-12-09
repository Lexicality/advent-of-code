pub mod aoc;
pub mod utils {
    pub mod coord2d;
    pub mod direction;
    pub mod error;
    pub mod grid;
    pub mod infgrid;
}

pub use crate::utils::coord2d::Coord2D;
pub use crate::utils::direction::Direction;
pub use crate::utils::error::AoCError;
pub use crate::utils::error::AoCResult;
pub use crate::utils::grid::Grid;
pub use crate::utils::infgrid::InfGrid;

pub type DataIn<'a> = &'a mut dyn Iterator<Item = String>;
pub type AoCDayFn = fn(DataIn) -> String;

pub struct AoCDay {
    pub year: &'static str,
    pub day: &'static str,
    pub func: AoCDayFn,
    pub example_func: Option<AoCDayFn>,
}

inventory::collect!(AoCDay);
