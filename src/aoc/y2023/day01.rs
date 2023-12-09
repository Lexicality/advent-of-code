use itertools::Itertools;

const NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];
const BACKNUMS: [&str; 18] = [
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn find_first(line: &str, nums: &[&str]) -> usize {
    let mut found_index = usize::MAX;
    let mut found_val = "";

    for num in nums {
        if let Some(index) = line.find(num) {
            if index < found_index {
                found_index = index;
                found_val = num;
                if index == 0 {
                    break;
                }
            }
        }
    }
    if found_val.is_empty() {
        panic!("Didn't find anything?!")
    } else if found_val.len() == 1 && found_val.chars().next().unwrap().is_ascii_digit() {
        found_val.parse().unwrap()
    } else {
        nums.iter().find_position(|a| **a == found_val).unwrap().0 + 1
    }
}

pub fn main(data: crate::DataIn) -> String {
    let mut total = 0;
    for line in data {
        let backline: String = line.chars().rev().collect();

        let first = find_first(&line, &NUMBERS);
        let last = find_first(&backline, &BACKNUMS);
        println!("{line} {first}{last}");
        let value = first * 10 + last;
        total += value;
    }
    format!("{total}")
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "1",
    func: main,
    example_func: None,
});
