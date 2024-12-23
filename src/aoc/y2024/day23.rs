// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type ComputerID = String;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let computers: HashMap<ComputerID, HashSet<ComputerID>> = {
        data.map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.to_owned(), b.to_owned())
        })
        .fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(a.clone()).or_default().insert(b.clone());
            acc.entry(b).or_default().insert(a);
            acc
        })
    };

    let ret = computers
        .iter()
        .permutations(2)
        .map(|v| {
            let (key_a, friends_a) = v[0];
            let (key_b, friends_b) = v[1];
            let mut common_friends: HashSet<_> = friends_a.intersection(friends_b).collect();
            common_friends.insert(key_a);
            common_friends.insert(key_b);
            common_friends
        })
        .filter(|cf| !cf.is_empty())
        .unique_by(|cf| cf.iter().sorted().join(","))
        .sorted_by_cached_key(|cf| cf.len())
        .filter(|common_friends| {
            let owned_friends: HashSet<_> =
                common_friends.iter().map(|v| (*v).to_owned()).collect();
            common_friends.iter().all(|key| {
                let mut argh: HashSet<_> = computers[*key].intersection(&owned_friends).collect();
                argh.insert(key);
                argh == *common_friends
            })
        })
        .map(|cf| cf.into_iter().sorted().join(","))
        .next_back()
        .unwrap();

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "23", main));
