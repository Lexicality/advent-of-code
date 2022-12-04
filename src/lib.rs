pub mod aoc;

pub type DataIn<'a> = &'a mut dyn Iterator<Item = String>;
pub type AoCDayFn = fn(DataIn) -> String;

pub struct AoCDay {
    pub year: &'static str,
    pub day: &'static str,
    pub func: AoCDayFn,
}

inventory::collect!(AoCDay);
// pub fn main(data: ) -> String {
