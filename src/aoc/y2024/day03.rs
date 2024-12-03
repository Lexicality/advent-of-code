use crate::AoCError;
use lazy_static::lazy_static;
use regex::Regex;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    }

    let mut ret = 0;
    let mut dont = false;
    for line in data {
        for capture in LINE_RE.captures_iter(&line) {
            if &capture[0] == "don't()" {
                dont = true;
            } else if &capture[0] == "do()" {
                dont = false;
            } else if !dont {
                let a: u64 = capture[1].parse().map_err(AoCError::new_from_parseerror)?;
                let b: u64 = capture[2].parse().map_err(AoCError::new_from_parseerror)?;
                ret += a * b;
            }
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "3", main));
