// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;
use std::collections::HashSet;

type Pocket = HashSet<char>;

fn get_priority(item: char) -> u32 {
    if !item.is_ascii_alphabetic() {
        panic!("Unknowable item: {}", item);
    }

    if item.is_ascii_uppercase() {
        return item as u32 - ('A' as u32 - 26 - 1);
    }
    item as u32 - ('a' as u32 - 1)
}

fn get_comomn_item<L: Iterator<Item = String>>(lines: L) -> Option<char> {
    let (e1, e2, e3) = lines.collect_tuple()?;
    let e1: Pocket = e1.chars().collect();
    let e2: Pocket = e2.chars().collect();
    let e3: Pocket = e3.chars().collect();

    let wat: Pocket = e1.intersection(&e2).copied().collect();

    let intersection: Vec<_> = e3.intersection(&wat).collect();

    if intersection.len() != 1 {
        panic!("Got multiple intersections: {:?}", intersection)
    }
    Some(*intersection[0])
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut total_prio = 0;
    for lines in &data.chunks(3) {
        if let Some(common_item) = get_comomn_item(lines) {
            let prio = get_priority(common_item);
            println!("{}/{}", common_item, prio);
            total_prio += prio;
        } else {
            panic!("sad");
        }
    }
    Ok(total_prio.to_string())
}

inventory::submit!(crate::AoCDay::mew("2022", "3", main));
