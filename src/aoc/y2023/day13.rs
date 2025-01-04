// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::cmp::Ordering;

use itertools::Itertools;

use crate::{CommonGrid, FlatGrid, Grid};

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut data = data.peekable();
    let mut ret = 0;
    while data.peek().is_some() {
        let lines = data
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().collect_vec());
        let grid: Grid<char> = Grid::new_from_lines(lines);
        println!("{grid}");
        // horizontal first
        let splitend = grid.width as usize;
        let mut potential_splits = (1..splitend).collect_vec();
        for row in 0..grid.height {
            let row = grid.get_row(row).into_iter().map(|(_, i)| *i).collect_vec();
            let mut row_rev = row.clone();
            row_rev.reverse();
            for split in potential_splits.drain(..).collect_vec() {
                let mut left = &row[..split];
                let mut right = &row_rev[..splitend - split];
                let llen = left.len();
                let rlen = right.len();
                match llen.cmp(&rlen) {
                    Ordering::Greater => {
                        left = &left[llen - rlen..];
                    }
                    Ordering::Less => {
                        right = &right[rlen - llen..];
                    }
                    Ordering::Equal => (),
                }
                if left == right {
                    potential_splits.push(split);
                }
                // println!("{} {left:?} {right:?} {}", split, left == right);
            }
        }
        match potential_splits.len() {
            1 => {
                let splittr = potential_splits[0];
                println!("Found vertical mirror at {}", splittr);
                ret += splittr;
                continue;
            }
            x if x > 1 => panic!("found {x} vertical mirrors!"),
            _ => {
                println!("No vertical mirrors");
            }
        }
        let splitend = grid.height as usize;
        let mut potential_splits = (1..splitend).collect_vec();
        for column in 0..grid.width {
            let col = grid
                .get_column(column)
                .into_iter()
                .map(|(_, i)| *i)
                .collect_vec();
            let mut col_rev = col.clone();
            col_rev.reverse();
            for split in potential_splits.drain(..).collect_vec() {
                let mut left = &col[..split];
                let mut right = &col_rev[..splitend - split];
                let llen = left.len();
                let rlen = right.len();
                match llen.cmp(&rlen) {
                    Ordering::Greater => {
                        left = &left[llen - rlen..];
                    }
                    Ordering::Less => {
                        right = &right[rlen - llen..];
                    }
                    Ordering::Equal => (),
                }
                if left == right {
                    potential_splits.push(split);
                }
                // println!("{} {left:?} {right:?} {}", split, left == right);
            }
        }
        match potential_splits.len() {
            1 => {
                let splittr = potential_splits[0];
                println!("Found horizontal mirror at {}", splittr);
                ret += splittr * 100;
                continue;
            }
            x => panic!("found {x} horizontal mirrors!"),
        }
    }
    Ok(ret.to_string())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MirrorRes {
    Void,
    Vertcial(usize),
    Horizontal(usize),
}

fn check_grid(grid: &Grid<char>, baseline: MirrorRes) -> MirrorRes {
    // horizontal first
    let splitend = grid.width as usize;
    let mut potential_splits = (1..splitend)
        .filter(|l| MirrorRes::Vertcial(*l) != baseline)
        .collect_vec();
    for row in 0..grid.height {
        let row = grid.get_row(row).into_iter().map(|(_, i)| *i).collect_vec();
        let mut row_rev = row.clone();
        row_rev.reverse();
        for split in potential_splits.drain(..).collect_vec() {
            let mut left = &row[..split];
            let mut right = &row_rev[..splitend - split];
            let llen = left.len();
            let rlen = right.len();
            match llen.cmp(&rlen) {
                Ordering::Greater => {
                    left = &left[llen - rlen..];
                }
                Ordering::Less => {
                    right = &right[rlen - llen..];
                }
                Ordering::Equal => (),
            }
            if left == right {
                potential_splits.push(split);
            }
            // println!("{} {left:?} {right:?} {}", split, left == right);
        }
    }
    match potential_splits.len() {
        1 => return MirrorRes::Vertcial(potential_splits[0]),
        x if x > 1 => return MirrorRes::Void,
        _ => (),
    }
    let splitend = grid.height as usize;
    let mut potential_splits = (1..splitend)
        .filter(|l| MirrorRes::Horizontal(*l) != baseline)
        .collect_vec();
    for column in 0..grid.width {
        let col = grid
            .get_column(column)
            .into_iter()
            .map(|(_, i)| *i)
            .collect_vec();
        let mut col_rev = col.clone();
        col_rev.reverse();
        for split in potential_splits.drain(..).collect_vec() {
            let mut left = &col[..split];
            let mut right = &col_rev[..splitend - split];
            let llen = left.len();
            let rlen = right.len();
            match llen.cmp(&rlen) {
                Ordering::Greater => {
                    left = &left[llen - rlen..];
                }
                Ordering::Less => {
                    right = &right[rlen - llen..];
                }
                Ordering::Equal => (),
            }
            if left == right {
                potential_splits.push(split);
            }
            // println!("{} {left:?} {right:?} {}", split, left == right);
        }
    }
    match potential_splits.len() {
        1 => MirrorRes::Horizontal(potential_splits[0]),
        _ => MirrorRes::Void,
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut data = data.peekable();
    let mut ret = 0;
    while data.peek().is_some() {
        let lines = data
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().collect_vec());
        let mut grid: Grid<char> = Grid::new_from_lines(lines);
        println!("{grid}");
        let baseline = check_grid(&grid, MirrorRes::Void);
        assert!(!matches!(baseline, MirrorRes::Void));
        println!("{baseline:?}");
        // chaos argh
        for coord in grid.keys() {
            let og = grid.get(&coord).copied().unwrap();
            let replace = match og {
                '.' => '#',
                '#' => '.',
                _ => unreachable!(),
            };
            grid.set(coord, replace);
            let smudged = check_grid(&grid, baseline);
            match smudged {
                MirrorRes::Horizontal(i) => {
                    println!("{coord} smudged to horizontal {i}");
                    ret += i * 100;
                    break;
                }
                MirrorRes::Vertcial(i) => {
                    println!("{coord} smudged to vertical {i}");
                    ret += i;
                    break;
                }
                _ => (),
            }
            grid.set(coord, og);
        }
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "13",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
