// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use super::computer::Computer;

const SPRINGCODE_PART_1: &str = "
NOT A T
NOT B J
OR T J
NOT C T
OR T J
AND D J
WALK
";

const SPRINGCODE_PART_2: &str = "
NOT A T
NOT B J
OR T J
NOT C T
OR T J
AND D J
NOT E T
AND F T
AND A T
AND B T
AND J T
AND I T
NOT T T
AND T J
RUN
";

pub fn run_droid(mut data: crate::DataIn, input: &'static str) -> crate::AoCResult<String> {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    computer.add_ascii_input(input.trim_start());
    computer.run_to_completion().unwrap();
    match computer.get_ascii_output() {
        Some(death) => Ok(death),
        None => {
            println!("{}", computer.get_ascii_lossy());
            Ok(computer.output.pop().unwrap().to_string())
        }
    }
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "21",
    part_1: crate::AoCPart {
        main: |data| run_droid(data, SPRINGCODE_PART_1),
        example: |_| unreachable!()
    },
    part_2: Some(crate::AoCPart {
        main: |data| run_droid(data, SPRINGCODE_PART_2),
        example: |_| unreachable!()
    }),
});
