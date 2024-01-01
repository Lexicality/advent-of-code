use std::collections::HashSet;

use itertools::Itertools;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret = data
        .filter(|line| {
            let trips = line.chars().tuple_windows().any(|(a, _, c)| a == c);
            if !trips {
                return false;
            }
            let unique_pairs: HashSet<_> = line
                .chars()
                .tuple_windows()
                .map(|(a, b)| format!("{a}{b}"))
                .collect();

            for pair in unique_pairs.into_iter() {
                let a = line.find(&pair);
                let b = line.rfind(&pair);
                if let (Some(first), Some(last)) = (a, b) {
                    if last > first + 1 {
                        return true;
                    }
                }
            }
            false
        })
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "5", main));
