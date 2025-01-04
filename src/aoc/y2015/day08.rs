// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    for line in data {
        let mut line = line.as_str();
        ret += line.len();
        line = line.strip_prefix('"').expect("Must have started with \"");
        line = line.strip_suffix('"').expect("Must have started with \"");
        let mut in_escape = false;
        let mut skip_n = 0;
        let mut strlen = 0;

        for c in line.chars() {
            if skip_n > 0 {
                skip_n -= 1;
                continue;
            }
            if !in_escape {
                if c == '\\' {
                    in_escape = true
                } else {
                    strlen += 1;
                }
            } else {
                in_escape = false;
                strlen += 1;
                if c == 'x' {
                    skip_n += 2;
                }
            }
        }

        ret -= strlen;
    }
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    for line in data {
        ret += line.escape_debug().to_string().len() + 2 - line.len();
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2015",
    day: "8",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
