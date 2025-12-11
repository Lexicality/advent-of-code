// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;

use cached::{UnboundCache, cached_key};
use itertools::Itertools;

use crate::AoCError;

type Key = u32;
type Store = HashMap<Key, Vec<Key>>;

const fn to_id(input: &str) -> Key {
    // all ids are 3 ascii letters, so 7 bits each.
    let input = input.as_bytes();
    let [a, b, c] = *input else {
        panic!("Code must be 3 bytes")
    };
    ((a as u32) << 16) + ((b as u32) << 8) + (c as u32)
}

const START: Key = to_id("you");
const END: Key = to_id("out");

fn parse_line(input: String) -> crate::AoCResult<(Key, Vec<Key>)> {
    let (gate, outputs) = input
        .split_once(": ")
        .ok_or_else(|| AoCError::new("Line {input} doesn't have a :"))?;

    Ok((to_id(gate), outputs.split_whitespace().map(to_id).collect()))
}

cached_key! {
    FIND_PATHS: UnboundCache<Key, usize> = UnboundCache::with_capacity(600);
    Key = { id };
    fn find_paths(id: Key, store: &Store) -> usize = {
        let my_outputs = &store[&id];
        log::debug!("Hello I'm {id} and my friends are {my_outputs:?}");
        my_outputs
            .iter()
            .copied()
            .map(|out_id| if out_id == END { 1 } else { find_paths(out_id, store)})
            .sum::<usize>()
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let store: Store = data.map(parse_line).try_collect()?;
    let ret = find_paths(START, &store);
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "11",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None
});
