// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

fn look_and_say(line: String) -> String {
    let mut ret = String::new();

    let mut cchar = line.chars().next().unwrap();
    let mut ccount = 0;
    for num in line.chars() {
        if num == cchar {
            ccount += 1;
        } else {
            ret += &ccount.to_string();
            ret.push(cchar);
            cchar = num;
            ccount = 1;
        }
    }
    ret += &ccount.to_string();
    ret.push(cchar);
    ret
}

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut line = data.next().unwrap();
    for _ in 0..50 {
        line = look_and_say(line);
    }
    Ok(line.len().to_string())
}

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    for line in data {
        println!("Input: {line}");
        let ret = look_and_say(line);
        println!("Output: {ret}");
    }
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay::mew_with_example(
    "2015",
    "10",
    main,
    main_example
));
