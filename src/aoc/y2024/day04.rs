// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use crate::{CharGrid, CommonGrid, Coord2D, Grid};

const UP_LEFT: Coord2D = Coord2D { x: -1, y: -1 };
const DOWN_LEFT: Coord2D = Coord2D { x: -1, y: 1 };
const UP_RIGHT: Coord2D = Coord2D { x: 1, y: -1 };
const DOWN_RIGHT: Coord2D = Coord2D { x: 1, y: 1 };

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
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

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "4",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
