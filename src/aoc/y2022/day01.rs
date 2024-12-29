// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut elves = vec![];
    let mut running_elf = 0;
    for line in data {
        let line = line.trim();
        if line.is_empty() {
            elves.push(running_elf);
            running_elf = 0;
            continue;
        }
        running_elf += line.parse::<i32>().unwrap();
    }
    if running_elf > 0 {
        elves.push(running_elf);
    }

    elves.sort_unstable();
    elves.reverse();

    Ok(elves[..3].iter().sum::<i32>().to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "1",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
