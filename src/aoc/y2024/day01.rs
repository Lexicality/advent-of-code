// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let (a, b): (Vec<_>, Vec<_>) = data
        .map(|line| -> (u64, u64) {
            let (a, b) = line.split_once(' ').expect("line must be splittable");
            (a.trim().parse().unwrap(), b.trim().parse().unwrap())
        })
        .unzip();

    let counts = b.into_iter().counts();
    Ok(a.into_iter()
        .map(|a| a * counts.get(&a).map(|b| *b as u64).unwrap_or_default())
        .reduce(u64::saturating_add)
        .unwrap()
        .to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "1", main));
