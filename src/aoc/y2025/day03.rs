// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

use crate::AoCError;

fn process_line_p1(line: String) -> crate::AoCResult<u64> {
    let mut current = '\0';
    let mut best: String = Default::default();

    log::debug!("Looking at bank {line}");

    for (i, char) in line.chars().enumerate() {
        if char <= current {
            continue;
        }
        current = char;

        match line.chars().skip(i + 1).max() {
            Some(friend) => best = [char, friend].into_iter().collect(),
            // end of the line
            None => break,
        }
        log::debug!("Found a new high pair {best}");
        // short circuit
        if char == '9' {
            // can't do better than 9
            log::debug!("Ending early!");
            break;
        }
    }

    log::debug!("The best is {best}!");

    best.parse().map_err(AoCError::new_from_parseerror)
}

fn process_line_p2(line: String) -> crate::AoCResult<u64> {
    log::debug!("Looking at bank {line}");
    let len = line.len();

    let (_, best) = (0..12).fold((0, vec![]), |(last_idx, mut acc): (usize, Vec<char>), i| {
        let endo = len - 11 + i;
        let line = &line[last_idx..endo];
        log::debug!("I'm character #{i} and I'm looking at {last_idx}..{endo}: {line}",);

        // stoopid
        let next_char = line.chars().max().expect("line won't be empty");
        let idx = line
            .chars()
            .position(|c| c == next_char)
            .expect("we know it's in there");

        log::debug!("Wow, my next best is {next_char} at {idx}!");

        acc.push(next_char);
        (idx + 1 + last_idx, acc)
    });

    let best: String = best.into_iter().collect();

    log::debug!("The best is {best}!");

    best.parse().map_err(AoCError::new_from_parseerror)
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let banks: Vec<_> = data.map(process_line_p1).try_collect()?;
    let ret: u64 = banks.into_iter().sum();
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let banks: Vec<_> = data.map(process_line_p2).try_collect()?;
    let ret: u64 = banks.into_iter().sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "3",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});
