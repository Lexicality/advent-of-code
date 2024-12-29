// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use md5::{Digest, Md5};

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let input = data.next().unwrap();
    let mut hasher = Md5::new();
    hasher.update(input);
    for i in (0..).map(|v| v.to_string()) {
        let mut hasher = hasher.clone();
        hasher.update(&i);
        let result = hasher.finalize();
        if result[0] == 0 && result[1] == 0 && result[2] == 0 {
            return Ok(i);
        }
    }
    unreachable!()
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "4",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::multi_line_example(data, part_2),
    }),
});
