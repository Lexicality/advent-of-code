// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

#[allow(unused_variables, unused_mut)]
pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let times = data.next().unwrap();
    let mut times = times.split_whitespace();
    let distances = data.next().unwrap();
    let mut distances = distances.split_whitespace();
    assert_eq!(times.next(), Some("Time:"));
    assert_eq!(distances.next(), Some("Distance:"));
    let mut ret: u64 = 0;
    let race_time = times.join("");
    let record = distances.join("");
    let race_time: u64 = race_time.parse().unwrap();
    let record: u64 = record.parse().unwrap();
    let mut hump = false;
    for hold_time in 0..race_time {
        let time = race_time - hold_time;
        let distance = hold_time * time;
        if distance < record && hump {
            break;
        } else if distance > record {
            hump = true;
            ret += 1;
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2023", "6", main));
