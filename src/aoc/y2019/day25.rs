// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fs::File;
use std::io::prelude::*;

use text_io::read;

use super::computer::{Computer, RunState};

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    while let RunState::NeedsInput = computer.run().unwrap() {
        print!("{}", computer.get_ascii_output().unwrap());
        computer.clear_output();
        print!("> ");
        let line: String = read!("{}\n");
        let line = line.trim().to_owned() + "\n";
        if line == "save\n" {
            let computer_value = ron::to_string(&computer).unwrap();
            File::options()
                .create(true)
                .truncate(true)
                .write(true)
                .open("25-memory-dump.txt")
                .unwrap()
                .write_all(&computer_value.into_bytes())
                .unwrap();
        } else if line == "load\n" {
            let mut raw_computer = String::new();
            File::open("25-memory-dump.txt")
                .unwrap()
                .read_to_string(&mut raw_computer)
                .unwrap();
            computer = ron::from_str(&raw_computer).unwrap();
        } else {
            computer.add_ascii_input(&line);
        }
    }
    Ok(computer.get_ascii_output().unwrap())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "25",
    part_1: crate::AoCPart {
        main: part_2,
        example: part_2
    },
    part_2: None,
});
