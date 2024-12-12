// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
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

inventory::submit!(crate::AoCDay::mew("2019", "1", main));
