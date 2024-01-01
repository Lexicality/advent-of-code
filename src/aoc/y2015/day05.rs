use itertools::Itertools;

const NAUGHTY_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: [char; 5] = ['a', 'o', 'e', 'u', 'i'];

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let ret = data
        .filter(|line| {
            for naughty in NAUGHTY_STRINGS.iter() {
                if line.contains(naughty) {
                    return false;
                }
            }
            let vowels = line.chars().filter(|c| VOWELS.contains(c)).count() >= 3;
            let doubles = line.chars().tuple_windows().any(|(a, b)| a == b);
            vowels && doubles
        })
        .count();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "5", main));
