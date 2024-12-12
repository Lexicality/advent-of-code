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

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret = data
        .filter(|line| {
            let trips = line.chars().tuple_windows().any(|(a, _, c)| a == c);
            if !trips {
                return false;
            }
            let unique_pairs: HashSet<_> = line
                .chars()
                .tuple_windows()
                .map(|(a, b)| format!("{a}{b}"))
                .collect();

            for pair in unique_pairs.into_iter() {
                let a = line.find(&pair);
                let b = line.rfind(&pair);
                if let (Some(first), Some(last)) = (a, b) {
                    if last > first + 1 {
                        return true;
                    }
                }
            }
            false
        })
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "5", main));
