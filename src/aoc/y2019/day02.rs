// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

use super::computer::Computer;

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    for line in data {
        let mut computer: Computer = line.parse().unwrap();
        computer.set(1, 12.into());
        computer.set(2, 2.into());
        computer.run_to_completion().unwrap();
        println!("{}", computer.get(&0))
    }
    Ok("".to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    for line in data {
        let og_computer: Computer = line.parse().unwrap();
        for (noun, verb) in (0..=99).cartesian_product(0..=99) {
            let mut computer = og_computer.clone();
            computer.set(1, noun.into());
            computer.set(2, verb.into());
            computer.run_to_completion().unwrap();
            let res = computer.get(&0).to_value();
            println!("{noun} {verb} {res}");
            if res == 19690720 {
                return Ok(((100 * noun) + verb).to_string());
            }
        }
    }
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "2",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1
    }),
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
