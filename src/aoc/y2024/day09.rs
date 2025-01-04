// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;
use num::Integer;

#[allow(dead_code)]
fn printarr(values: &[(usize, Vec<usize>)]) {
    for value in values.iter().flat_map(|(_, v)| v) {
        if *value == usize::MAX {
            print!(".");
        } else {
            print!("{value}");
        }
    }
    println!();
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
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

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut blanks: Vec<(usize, usize)> = Vec::new();
    let mut block_id = 0_usize;
    let mut arr: Vec<(usize, Vec<usize>)> = data
        .next()
        .unwrap()
        .chars()
        .map(|c| char::to_digit(c, 10).unwrap().try_into().unwrap())
        .enumerate()
        .map(|(idx, n)| {
            let is_space = idx.is_odd();
            let value = if is_space {
                blanks.push((idx, n));
                usize::MAX
            } else {
                let tmp = block_id;
                block_id += 1;
                tmp
            };

            (idx, [value].repeat(n))
        })
        .collect();
    // DEBUG
    // printarr(&arr);

    let bastard = arr
        .clone()
        .into_iter()
        .map(|(id, v)| (id, v.len()))
        .filter(|(id, len)| *len > 0 && id.is_even())
        .rev();

    for (file_id, file_len) in bastard {
        let (file_idx, _) = arr.iter().find_position(|(id, _)| *id == file_id).unwrap();
        // println!("Looking at file at {file_idx}");

        let Some((blank_idx, blank_len)) = arr
            .iter()
            .enumerate()
            .filter(|(_, (id, _))| id.is_odd())
            .map(|(idx, (_, v))| (idx, v.len()))
            .find(|(idx, len)| *len >= file_len && *idx < file_idx)
        else {
            // println!("no blanks available");
            continue;
        };
        // println!("Found a blank of length {blank_len} at {blank_idx}");

        if blank_len > file_len {
            // the blank we want to swap is too big, make it smaller
            let blank = arr.get_mut(blank_idx).unwrap();
            blank.1.truncate(file_len);
        }

        // swap 'em
        arr.swap(file_idx, blank_idx);

        if blank_len > file_len {
            // We need to insert a new blank to make up lost space
            arr.insert(
                blank_idx + 1,
                (9999999, [usize::MAX].repeat(blank_len - file_len)),
            );
        }

        // printarr(&arr);
    }

    let ret: usize = arr
        .into_iter()
        .flat_map(|(_, v)| v)
        .enumerate()
        .filter(|(_, v)| *v != usize::MAX)
        .map(|(a, b)| a * b)
        .sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "9",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::multi_line_example(data, part_2),
    }),
});
