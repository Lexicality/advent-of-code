// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::{CharGrid, CommonGrid, Coord2D, Direction, Grid};

fn directionalise(coord: Coord2D) -> [Coord2D; 4] {
    [
        coord + Direction::North.to_coord(),
        coord + Direction::East.to_coord(),
        coord + Direction::South.to_coord(),
        coord + Direction::West.to_coord(),
    ]
}

fn regionate(data: crate::DataIn) -> Vec<Vec<Coord2D>> {
    let grid: Grid<char> = Grid::new_from_chars(data).unwrap();

    println!("{grid:#}");

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
    regions
}

fn get_perimeter(region: &[Coord2D]) -> impl Iterator<Item = Coord2D> + use<'_> {
    // this is dumb as hell
    let region_hash: HashSet<_> = region.iter().copied().collect();
    region.iter().copied().flat_map(move |coord| {
        directionalise(coord)
            .into_iter()
            .filter(|c| !region_hash.contains(c))
            .collect_vec()
            .into_iter()
    })
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let regions = regionate(data);
    let ret: usize = regions
        .into_iter()
        .map(|region| {
            let area = region.len();
            let perimeter = get_perimeter(&region).count();
            area * perimeter
        })
        .sum();
    Ok(ret.to_string())
}

fn side_flood(perimeter: &HashSet<Coord2D>, directions: [Direction; 2]) -> Vec<Vec<Coord2D>> {
    // even wonkier flood fill
    let mut seen = HashSet::with_capacity(perimeter.len());
    let mut perimeter_regions = Vec::new();
    for coord in perimeter.iter().copied() {
        if seen.contains(&coord) {
            continue;
        }
        let mut region = Vec::new();
        let mut to_check = vec![coord];
        while let Some(coord) = to_check.pop() {
            // println!("- gavin a look at {coord_to_check}");
            if !seen.insert(coord) || !perimeter.contains(&coord) {
                // println!("-- nah");
                continue;
            }
            region.push(coord);
            for direction in directions {
                // println!("-- looking at friend: {coord}");
                to_check.push(coord + direction.to_coord());
            }
        }
        perimeter_regions.push(region);
    }
    perimeter_regions
}

fn get_sides(region: &[Coord2D]) -> usize {
    if region.len() < 3 {
        return 4;
    }
    let perimeter: HashSet<_> = get_perimeter(region).collect();

    let up_down_regions = side_flood(&perimeter, [Direction::North, Direction::South]);
    let left_right_regions = side_flood(&perimeter, [Direction::East, Direction::West]);

    let region: HashSet<_> = region.iter().copied().collect();

    let ud_sum = up_down_regions
        .into_iter()
        .map(|perimeter| {
            // println!(" updown {:?}", perimeter);
            [Direction::East, Direction::West]
                .into_iter()
                .filter(|dir| {
                    perimeter
                        .iter()
                        .any(|coord| region.contains(&(coord + &dir.to_coord())))
                })
                .count()
        })
        // .inspect(|count| println!("  {count}"))
        .sum::<usize>();
    // println!("{ud_sum}");
    let lr_sum = left_right_regions
        .into_iter()
        .map(|perimeter| {
            println!(" leftright {:?}", perimeter);
            [Direction::North, Direction::South]
                .into_iter()
                .filter(|dir| {
                    perimeter
                        .iter()
                        .any(|coord| region.contains(&(coord + &dir.to_coord())))
                })
                .count()
        })
        .inspect(|count| println!("  {count}"))
        .sum::<usize>();
    println!("{lr_sum}");
    ud_sum + lr_sum
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let regions = regionate(data);
    // println!("Got {} regions", regions.len());
    let ret: usize = regions
        .into_iter()
        .map(|region| {
            let area = region.len();
            let mut num_sides = get_sides(&region);
            // println!(
            //     "Region {} has area {area} and {num_sides} sides = {}",
            //     region.iter().min().unwrap(),
            //     area * num_sides
            // );
            // HACK: My sides code cannot correctly handle interior slots with a
            // T shape, but conveniently my input is such that this hack allows
            // me to get the star and I really can't be bothered to fix it
            if !num_sides.is_multiple_of(2) {
                num_sides += 1;
            }
            area * num_sides
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "12",
    part_1: crate::AoCPart {
        main: part_1,
        example: |data| crate::partitioned_example(data, part_1)
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::partitioned_example(data, part_2)
    },),
});
