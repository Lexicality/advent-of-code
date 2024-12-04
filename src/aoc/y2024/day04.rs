use crate::{CharGrid, CommonGrid, Grid};

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: Grid<char> = Grid::new_from_chars(data).unwrap();
    let ret = grid
        .iter()
        .filter(|(_, c)| **c == 'X')
        .flat_map(|(x_coord, _)| {
            grid.get_neighbour_coords_filtered(*x_coord, true, |_, v| *v == 'M')
                .map(|m_coord| {
                    let delta = m_coord - *x_coord;
                    (m_coord + delta, m_coord + (delta * 2).unwrap())
                })
        })
        .filter(|(a_coord, s_coord)| {
            matches!(
                (grid.get(a_coord), grid.get(s_coord)),
                (Some('A'), Some('S'))
            )
        })
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "4", main));
