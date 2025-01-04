// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

struct Range {
    start: u32,
    end: u32,
}
impl Range {
    fn new(input: &str) -> Range {
        let (start, end) = input.split_once('-').unwrap();
        Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn intersects(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut overlap = 0;
    for pair in data {
        let (elf1, elf2) = pair.split_once(',').unwrap();
        let elf1 = Range::new(elf1);
        let elf2 = Range::new(elf2);
        if elf1.contains(&elf2) || elf2.contains(&elf1) {
            overlap += 1;
        }
    }
    Ok(overlap.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut overlap = 0;
    for pair in data {
        let (elf1, elf2) = pair.split_once(',').unwrap();
        let elf1 = Range::new(elf1);
        let elf2 = Range::new(elf2);
        if elf1.intersects(&elf2) || elf2.intersects(&elf1) {
            overlap += 1;
        }
    }
    Ok(overlap.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "4",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
