// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use super::computer;

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "5",
    part_1: Some(crate::AoCPart {
        main: |data| computer::run_one_with_input(data, &[1]),
        example: |data| crate::multi_line_example(data, computer::run_one)
    }),
    part_2: Some(crate::AoCPart {
        main: |data| computer::run_one_with_input(data, &[5]),
        example: |data| crate::multi_line_example(data, computer::run_one)
    }),
});
