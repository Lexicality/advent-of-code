pub mod aoc;
pub mod utils {
    pub mod coord2d;
    pub mod direction;
    pub mod grid;
}

pub use crate::utils::coord2d::Coord2D;
pub use crate::utils::direction::Direction;
pub use crate::utils::grid::Grid;

pub type DataIn<'a> = &'a mut dyn Iterator<Item = String>;
pub type AoCDayFn = fn(DataIn) -> String;

pub struct AoCDay {
    pub year: &'static str,
    pub day: &'static str,
    pub func: AoCDayFn,
}

inventory::collect!(AoCDay);
