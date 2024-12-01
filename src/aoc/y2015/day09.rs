use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut destinations = HashSet::new();

    let routes = data
        .map(|line| {
            let (from, line) = line.split_once(" to ").unwrap();
            let (to, distance) = line.split_once(" = ").unwrap();
            destinations.insert(from.to_owned());
            destinations.insert(to.to_owned());
            (
                from.to_owned(),
                to.to_owned(),
                distance.parse::<u64>().unwrap(),
            )
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, HashMap<String, u64>>, (from, to, distance)| {
                acc.entry(from.clone())
                    .or_default()
                    .insert(to.clone(), distance);
                acc.entry(to).or_default().insert(from, distance);
                acc
            },
        );

    let ret: u64 = destinations
        .iter()
        .permutations(destinations.len())
        .map(|route| {
            route
                .into_iter()
                .tuple_windows()
                .map(|(from, to)| routes[from][to])
                .sum()
        })
        .max()
        .unwrap();

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "9", main));
