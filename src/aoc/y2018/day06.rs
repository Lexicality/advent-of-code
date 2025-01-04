// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashSet;

use itertools::Itertools;

use crate::{Coord2D, Coordinate};

fn get_closest(coord: Coord2D, coords: &[Coord2D]) -> Option<Coord2D> {
    let ((a, a_dist), (_, b_dist)) = coords
        .iter()
        .map(|other| (other, coord.distance(other)))
        .sorted_by_cached_key(|(_, dist)| *dist)
        .next_tuple()
        .unwrap();
    (a_dist != b_dist).then_some(*a)
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let coords: Vec<Coord2D> = data.map(|line| line.parse()).try_collect()?;
    let min = coords
        .iter()
        .copied()
        .reduce(|acc, coord| acc.get_min(&coord))
        .unwrap();
    let max = coords
        .iter()
        .copied()
        .reduce(|acc, coord| acc.get_max(&coord))
        .unwrap();

    let mut to_ignore = HashSet::with_capacity(coords.len());

    // I love iterators so much
    let ret = (min.x..=max.x)
        .cartesian_product(min.y..=max.y)
        .filter_map(|to_check| {
            let to_check = to_check.into();
            let closest = get_closest(to_check, &coords)?;
            if to_ignore.contains(&closest) {
                None
            } else if to_check.x == min.x
                || to_check.x == max.x
                || to_check.y == min.y
                || to_check.y == max.y
            {
                to_ignore.insert(closest);
                None
            } else {
                Some(closest)
            }
        })
        .collect_vec()
        .into_iter()
        .filter(|closest| !to_ignore.contains(closest))
        .sorted()
        .chunk_by(|closest| *closest)
        .into_iter()
        .map(|(_, group)| group.count())
        .sorted()
        .next_back()
        .unwrap();

    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn, max_distance: u64) -> crate::AoCResult<String> {
    let coords: Vec<Coord2D> = data.map(|line| line.parse()).try_collect()?;
    let min = coords
        .iter()
        .copied()
        .reduce(|acc, coord| acc.get_min(&coord))
        .unwrap();
    let max = coords
        .iter()
        .copied()
        .reduce(|acc, coord| acc.get_max(&coord))
        .unwrap();

    let ret = (min.x..=max.x)
        .cartesian_product(min.y..=max.y)
        .filter(|to_check| {
            let to_check = to_check.into();
            let distance = coords
                .iter()
                .map(|coord| coord.distance(&to_check) as u64)
                .sum::<u64>();
            distance < max_distance
        })
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2018",
    day: "6",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1
    }),
    part_2: Some(crate::AoCPart {
        main: |data| part_2(data, 10_000),
        example: |data| part_2(data, 32),
    }),
});
