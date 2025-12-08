// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

use crate::Coord3D;

// now you're just being intentionally anonying ¬_¬
fn dist(a: Coord3D, b: Coord3D) -> i64 {
    (a.x as i64 - b.x as i64).pow(2)
        + (a.y as i64 - b.y as i64).pow(2)
        + (a.z as i64 - b.z as i64).pow(2)
}

fn get_distances(coords: Vec<Coord3D>) -> Vec<(i64, (Coord3D, Coord3D))> {
    coords
        .into_iter()
        .array_combinations()
        .map(|[a, b]| (dist(a, b), (a, b)))
        .sorted_unstable()
        .collect()
}

pub fn part_1(data: crate::DataIn, num_connections: usize) -> crate::AoCResult<String> {
    let coords: Vec<Coord3D> = data.map(|line| line.parse()).try_collect()?;
    let distances = get_distances(coords);

    log::trace!("distances! {distances:?}");

    let mut circuits: Vec<HashSet<Coord3D>> = Vec::with_capacity(num_connections);

    for (distance, (a, b)) in distances[0..num_connections].iter() {
        log::debug!("Looking at pair {a}, {b} with distance {distance}!");

        let a_circuit = circuits
            .iter()
            .find_position(|col| col.contains(a))
            .map(|(pos, _)| pos);
        let b_circuit = circuits
            .iter()
            .find_position(|col| col.contains(b))
            .map(|(pos, _)| pos);
        match (a_circuit, b_circuit) {
            (None, None) => {
                log::debug!("New circuit time");
                circuits.push([a, b].into_iter().copied().collect())
            }
            (Some(pos), None) => {
                log::debug!("a is already in a circuit, adding b");
                circuits[pos].insert(*b);
            }
            (None, Some(pos)) => {
                log::debug!("b is already in a circuit, adding a");
                circuits[pos].insert(*a);
            }
            (Some(pos_a), Some(pos_b)) => {
                let (winner, loser) = match pos_a.cmp(&pos_b) {
                    Ordering::Less => (pos_a, pos_b),
                    Ordering::Equal => {
                        log::debug!("They're already in the same circuit!");
                        // both in same circuit
                        continue;
                    }
                    Ordering::Greater => (pos_b, pos_a),
                };
                log::debug!("Both in separate circuits, merging!");
                let loser = circuits.swap_remove(loser);
                circuits[winner].extend(loser);
            }
        }
    }

    log::debug!("There are {} circuits!", circuits.len());

    let ret: usize = circuits
        .into_iter()
        .map(|circuit| circuit.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .inspect(|c| log::debug!("Final circuit size {c}"))
        .product();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let coords: Vec<Coord3D> = data.map(|line| line.parse()).try_collect()?;
    let mut circuits: Vec<HashSet<Coord3D>> = coords
        .iter()
        .map(|coord| [*coord].into_iter().collect())
        .collect();
    let distances = get_distances(coords);

    log::trace!("distances! {distances:?}");

    let mut ret = 0;

    for (distance, (a, b)) in distances.iter() {
        log::debug!("Looking at pair {a}, {b} with distance {distance}!");

        let a_circuit = circuits
            .iter()
            .find_position(|col| col.contains(a))
            .map(|(pos, _)| pos);
        let b_circuit = circuits
            .iter()
            .find_position(|col| col.contains(b))
            .map(|(pos, _)| pos);
        match (a_circuit, b_circuit) {
            (None, None) => {
                log::debug!("New circuit time");
                circuits.push([a, b].into_iter().copied().collect())
            }
            (Some(pos), None) => {
                log::debug!("a is already in a circuit, adding b");
                circuits[pos].insert(*b);
            }
            (None, Some(pos)) => {
                log::debug!("b is already in a circuit, adding a");
                circuits[pos].insert(*a);
            }
            (Some(pos_a), Some(pos_b)) => {
                let (winner, loser) = match pos_a.cmp(&pos_b) {
                    Ordering::Less => (pos_a, pos_b),
                    Ordering::Equal => {
                        log::debug!("They're already in the same circuit!");
                        // both in same circuit
                        continue;
                    }
                    Ordering::Greater => (pos_b, pos_a),
                };
                log::debug!("Both in separate circuits, merging!");
                let loser = circuits.swap_remove(loser);
                circuits[winner].extend(loser);
                if circuits.len() == 1 {
                    log::debug!("Fully merginated!");
                    // we're fully merged
                    ret = a.x as i64 * b.x as i64;
                    break;
                }
            }
        }
    }

    // log::debug!("There are {} circuits!", circuits.len());
    //
    // let ret: usize = circuits
    //     .into_iter()
    //     .map(|circuit| circuit.len())
    //     .sorted_unstable()
    //     .rev()
    //     .take(3)
    //     .inspect(|c| log::debug!("Final circuit size {c}"))
    //     .product();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "8",
    part_1: crate::AoCPart {
        main: |data| part_1(data, 1000),
        example: |data| part_1(data, 10)
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});
