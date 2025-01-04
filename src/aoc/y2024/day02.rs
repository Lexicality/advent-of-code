// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

enum Direction {
    Mystery,
    Increasing,
    Decreasing,
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    'lines: for line in data {
        print!("{line} ");
        let mut dir = Direction::Mystery;
        for (a, b) in line
            .split(' ')
            .map(|c| c.parse::<u32>().unwrap())
            .tuple_windows()
        {
            if a.abs_diff(b) > 3 {
                println!("UNSAFE (too big a change)");
                continue 'lines;
            }
            match a.cmp(&b) {
                std::cmp::Ordering::Less => {
                    if matches!(dir, Direction::Increasing) {
                        println!("UNSAFE (dir change)");
                        continue 'lines;
                    }
                    dir = Direction::Decreasing
                }
                std::cmp::Ordering::Equal => {
                    println!("UNSAFE (equality)");
                    continue 'lines;
                }
                std::cmp::Ordering::Greater => {
                    if matches!(dir, Direction::Decreasing) {
                        println!("UNSAFE (dir change)");
                        continue 'lines;
                    }
                    dir = Direction::Increasing
                }
            }
        }
        println!("safe!");
        ret += 1;
    }
    Ok(ret.to_string())
}

fn check_line(mut results: impl Iterator<Item = u32>) -> bool {
    let mut dir = Direction::Mystery;
    let mut a = results.next().unwrap();
    for b in results {
        if a.abs_diff(b) > 3 {
            return false;
        }
        match a.cmp(&b) {
            std::cmp::Ordering::Less => {
                if matches!(dir, Direction::Increasing) {
                    return false;
                }
                dir = Direction::Decreasing
            }
            std::cmp::Ordering::Equal => {
                return false;
            }
            std::cmp::Ordering::Greater => {
                if matches!(dir, Direction::Decreasing) {
                    return false;
                }
                dir = Direction::Increasing
            }
        }
        a = b;
    }
    true
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    'lines: for line in data {
        let results: Vec<u32> = line.split(' ').map(|c| c.parse().unwrap()).collect();
        if check_line(results.iter().copied()) {
            ret += 1;
            continue;
        }
        for to_remove in 0..results.len() {
            if check_line(
                results
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != to_remove)
                    .map(|(_, v)| v)
                    .copied(),
            ) {
                ret += 1;
                continue 'lines;
            }
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "2",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
