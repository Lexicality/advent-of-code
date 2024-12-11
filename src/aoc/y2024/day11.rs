use itertools::Itertools;
use num::Integer;

use crate::AoCError;

type Aa = u64;

const ITERATIONS: usize = 25;

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut stones: Vec<Aa> = data
        .next()
        .unwrap()
        .split(" ")
        .map(str::parse)
        .try_collect()
        .map_err(AoCError::new_from_parseerror)?;

    println!("{stones:?}");

    for _ in 0..ITERATIONS {
        stones = stones
            .into_iter()
            .flat_map(|num| {
                if num == 0 {
                    return vec![Ok(1)].into_iter();
                }
                let strval = num.to_string();
                if strval.len().is_even() {
                    let (a, b) = strval.split_at(strval.len() / 2);
                    vec![a.parse(), b.parse()].into_iter()
                } else {
                    vec![Ok(num * 2024)].into_iter()
                }
            })
            .try_collect()
            .map_err(AoCError::new_from_parseerror)?;
        println!("{stones:?}");
    }

    let ret = stones.len();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "11", main));
