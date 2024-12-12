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
    let ret: u32 = data
        .map(|line| {
            let mut tmp: Vec<u32> = line.split('x').map(|c| c.parse()).try_collect().unwrap();
            tmp.sort();
            let (l, w, h) = tmp.into_iter().collect_tuple().unwrap();

            let a = (l, w);
            let b = (w, h);
            let c = (l, h);
            let slack = a.min(b).min(c);

            2 * slack.0 + 2 * slack.1 + l * w * h
        })
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "2", main));
