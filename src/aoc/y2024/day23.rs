// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashMap, HashSet};

type ComputerID = String;
type HashKey = [ComputerID; 3];

fn hash_key(a: &ComputerID, b: &ComputerID, c: &ComputerID) -> HashKey {
    let mut ret = [a, b, c];
    ret.sort();
    ret.map(String::to_owned)
}

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

    let mut seen: HashSet<HashKey> = HashSet::with_capacity(computers.len());

    let ret: usize = computers
        .iter()
        .filter(|(key, friends)| key.starts_with('t') && friends.len() >= 2)
        .map(|(me, friends)| {
            let mut to_compare = friends.clone();
            friends
                .iter()
                .map(|friend| {
                    to_compare.remove(friend);
                    let friend_friends = &computers[friend];
                    friend_friends
                        .intersection(&to_compare)
                        .filter(|ff| seen.insert(hash_key(me, friend, *ff)))
                        .inspect(|ff| println!("{me},{friend},{ff}"))
                        .count()
                })
                .sum::<usize>()
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "23", main));
