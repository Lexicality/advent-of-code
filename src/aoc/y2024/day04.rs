use crate::{CharGrid, CommonGrid, Coord2D, Grid};

const UP_LEFT: Coord2D = Coord2D { x: -1, y: -1 };
const DOWN_LEFT: Coord2D = Coord2D { x: -1, y: 1 };
const UP_RIGHT: Coord2D = Coord2D { x: 1, y: -1 };
const DOWN_RIGHT: Coord2D = Coord2D { x: 1, y: 1 };

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: Grid<char> = Grid::new_from_chars(data).unwrap();
    let ret = grid
        .iter()
        .filter(|(_, c)| **c == 'A')
        .filter(|(a_coord, _)| {
            matches!(
                (
                    grid.get(&(**a_coord + UP_LEFT)),
                    grid.get(&(**a_coord + DOWN_LEFT)),
                    grid.get(&(**a_coord + UP_RIGHT)),
                    grid.get(&(**a_coord + DOWN_RIGHT)),
                ),
                (Some('M'), Some('M'), Some('S'), Some('S'))
                    | (Some('S'), Some('S'), Some('M'), Some('M'))
                    | (Some('M'), Some('S'), Some('M'), Some('S'))
                    | (Some('S'), Some('M'), Some('S'), Some('M'))
            )
        })
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "4", main));
