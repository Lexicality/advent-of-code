// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashSet, VecDeque};

use crate::{CharGrid, CommonGrid, Coord2D, Direction, Grid};

fn regionate(coord: Coord2D) -> [Coord2D; 4] {
    [
        coord + Direction::North.to_coord(),
        coord + Direction::East.to_coord(),
        coord + Direction::South.to_coord(),
        coord + Direction::West.to_coord(),
    ]
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: Grid<char> = Grid::new_from_chars(data).unwrap();
    // super wonky floodfill
    let mut regions = Vec::new();
    let mut seen = HashSet::with_capacity(grid.len());
    for (starting_coord, region_value) in grid.iter() {
        if seen.contains(starting_coord) {
            continue;
        }
        let mut queue = VecDeque::new();
        queue.push_back(*starting_coord);
        let mut region = Vec::new();
        while let Some(queue_coord) = queue.pop_front() {
            if !seen.insert(queue_coord) {
                continue;
            }
            region.push(queue_coord);
            queue.extend(
                grid.get_neighbour_coords_filtered(queue_coord, false, |coord, value| {
                    !seen.contains(coord) && value == region_value
                }),
            );
        }
        regions.push(region);
    }

    let ret: usize = regions
        .into_iter()
        .map(|region| {
            let area = region.len();
            // this is dumb as hell
            let region: HashSet<_> = region.into_iter().collect();
            let perimeter: usize = region
                .iter()
                .map(|coord| {
                    regionate(*coord)
                        .into_iter()
                        .filter(|c| !region.contains(c))
                        .count()
                })
                .sum();
            area * perimeter
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "12",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None,
});
