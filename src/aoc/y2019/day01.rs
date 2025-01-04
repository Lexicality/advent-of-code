// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret: u64 = 0;
    for line in data {
        let num: u64 = line.parse().unwrap();
        let req = (num / 3) - 2;
        println!("Input: {num} Required: {req}");
        ret += req;
    }
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret: u64 = 0;
    for line in data {
        let num: u64 = line.parse().unwrap();
        let mut weight = num;
        let mut req = 0;
        loop {
            weight = (weight / 3).saturating_sub(2);
            if weight == 0 {
                break;
            }
            req += weight;
        }
        println!("Input: {num} Required: {req}");
        ret += req;
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "1",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
