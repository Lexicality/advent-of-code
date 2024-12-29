// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use super::computer::Computer;

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    for line in data {
        let mut computer: Computer = line.parse().unwrap();
        computer.run_to_completion().unwrap();
        println!("---------");
    }
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "9",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1
    }),
    part_2: None,
});
