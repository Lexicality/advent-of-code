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

const YOU: Key = to_id("you");
const SERVER: Key = to_id("svr");
const FFT: Key = to_id("fft");
const DAC: Key = to_id("dac");
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
    let ret = find_paths(YOU, &store);
    Ok(ret.to_string())
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Default)]
struct PathState {
    found_fft: bool,
    found_dac: bool,
}

impl PathState {
    fn get_score(&self) -> usize {
        if self.found_dac && self.found_fft {
            1
        } else {
            0
        }
    }
}

cached_key! {
    FIND_PATHS_2: UnboundCache<(Key, PathState), usize> = UnboundCache::with_capacity(600);
    Key = { (id, path_state) };
    fn find_paths_2(id: Key, path_state: PathState, store: &Store) -> usize = {
        let mut path_state = path_state;
        if id == FFT {
            path_state.found_fft = true;
        } else if id == DAC{
            path_state.found_dac = true;
        }
        let my_outputs = &store[&id];
        log::debug!("Hello I'm {id} and my friends are {my_outputs:?}");

        my_outputs
            .iter()
            .copied()
            .map(|out_id| {
                if out_id == END {
                    path_state.get_score()
                } else {
                    find_paths_2(out_id, path_state, store)
                }
            })
            .sum::<usize>()
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let store: Store = data.map(parse_line).try_collect()?;
    let ret = find_paths_2(SERVER, Default::default(), &store);
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "11",
    part_1: crate::AoCPart {
        main: part_1,
        example: |data| {
            let (data, _) = crate::partition_input(data);
            part_1(data)
        }
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| {
            let (_, data) = crate::partition_input(data);
            part_2(data)
        }
    })
});
