// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.
use itertools::Itertools;

fn main(mut data: crate::DataIn, window_size: usize) -> crate::AoCResult<String> {
    let chars = data.next().unwrap().chars().collect_vec();
    for i in 0.. {
        let end = i + window_size;
        if chars[i..end].iter().unique().count() == window_size {
            return Ok(end.to_string());
        }
    }
    unreachable!()
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "6",
    part_1: Some(crate::AoCPart {
        main: |data| main(data, 4),
        example: |data| crate::multi_line_example(data, |data| main(data, 4))
    }),
    part_2: Some(crate::AoCPart {
        main: |data| main(data, 14),
        example: |data| crate::multi_line_example(data, |data| main(data, 14))
    }),
});
