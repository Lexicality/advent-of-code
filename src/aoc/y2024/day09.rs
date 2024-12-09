use itertools::Itertools;
use num::Integer;

#[allow(dead_code)]
fn printarr(values: &[usize]) {
    for value in values {
        if *value == usize::MAX {
            print!(".");
        } else {
            print!("{value}");
        }
    }
    println!();
}

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut blanks: Vec<(usize, usize)> = Vec::new();
    let mut blank_idx = 0_usize;
    let mut block_id = 0_usize;
    let mut arr: Vec<usize> = data
        .next()
        .unwrap()
        .chars()
        .map(|c| char::to_digit(c, 10).unwrap().try_into().unwrap())
        .enumerate()
        .flat_map(|(idx, n)| {
            let is_space = idx.is_odd();
            let value = if is_space {
                blanks.push((blank_idx, n));
                blank_idx += n;

                usize::MAX
            } else {
                blank_idx += n;

                let tmp = block_id;
                block_id += 1;
                tmp
            };

            [value].repeat(n)
        })
        .collect();
    // DEBUG
    // printarr(&arr);

    // Purely out of concern that part 2 will change the "which bit is space"
    // rule, I'm going to do a search for the last value rather than assuming
    // it's the last index
    let mut rearidx = arr.len()
        - 1
        - arr
            .iter()
            .rev()
            .find_position(|v| **v != usize::MAX)
            .unwrap()
            .0;

    'outer: for (idx, len) in blanks {
        for offset in 0..len {
            while arr[rearidx] == usize::MAX {
                if rearidx == 0 {
                    // println!("nothing left to swap?!");
                    break 'outer;
                }
                rearidx -= 1;
            }
            let swaptarget = idx + offset;
            if swaptarget >= rearidx {
                // println!("overshoot!");
                break 'outer;
            }
            arr.swap(idx + offset, rearidx);
            rearidx -= 1;
            // DEBUG
            // printarr(&arr);
        }
    }

    let ret: usize = arr
        .into_iter()
        .filter(|v| *v != usize::MAX)
        .enumerate()
        .map(|(a, b)| a * b)
        .sum();
    Ok(ret.to_string())
}

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    crate::multi_line_example(data, main)
}

inventory::submit!(crate::AoCDay::mew_with_example(
    "2024",
    "9",
    main,
    main_example
));
