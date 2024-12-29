// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashSet, VecDeque};
use std::str::Chars;

#[derive(Debug)]
struct SubRoutine(VecDeque<char>, HashSet<char>);

impl SubRoutine {
    fn new(chars: &mut Chars) -> SubRoutine {
        SubRoutine(chars.take(14).collect(), HashSet::with_capacity(14))
    }

    fn bingo_bango(&mut self) -> bool {
        self.1.clear();
        for c in self.0.iter() {
            self.1.insert(*c);
        }
        self.1.len() == 14
    }

    fn rotate(&mut self, value: char) {
        self.0.pop_front().unwrap();
        self.0.push_back(value);
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    for line in data {
        let mut citer = line.chars();
        let mut sub = SubRoutine::new(&mut citer);
        println!("{:?}", sub);
        if sub.bingo_bango() {
            println!("14");
            continue;
        }
        let mut i = 15;
        for c in citer {
            sub.rotate(c);
            if sub.bingo_bango() {
                break;
            }
            i += 1
        }
        println!("{}", i);
    }
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "6",
    part_1: None,
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
