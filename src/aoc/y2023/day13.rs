use std::cmp::Ordering;

use itertools::Itertools;

use crate::Grid;

pub fn main(data: crate::DataIn) -> String {
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
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "13",
    func: main,
    example_func: None,
});
