use crate::AoCError;
use lazy_static::lazy_static;
use regex::Regex;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    }

    let mut ret = 0;
    for line in data {
        for [a, b] in LINE_RE.captures_iter(&line).map(|c| c.extract().1) {
            let a: u64 = a.parse().map_err(AoCError::new_from_parseerror)?;
            let b: u64 = b.parse().map_err(AoCError::new_from_parseerror)?;
            ret += a * b;
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "3", main));
