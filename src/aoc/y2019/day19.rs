// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

use crate::Grid;

use super::computer::Computer;

enum GridState {
    Void,
    Tractor,
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Void => '·',
            GridState::Tractor => '░',
        }
        .fmt(f)
    }
}

fn mode(numbers: &[f64]) -> Option<f64> {
    let mut counts = HashMap::new();

    numbers.iter().copied().max_by_key(|&n| {
        let count = counts.entry((n * 1000.0) as i64).or_insert(0);
        *count += 1;
        *count
    })
}

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let base_computer: Computer = data.next().unwrap().parse().unwrap();

    let grid: Grid<GridState> = (0..50)
        .cartesian_product(0..50)
        .map(|(x, y)| {
            let mut computer = base_computer.clone();
            computer.input.push_back(x);
            computer.input.push_back(y);
            computer
                .run_to_completion()
                .expect("The computer must work");
            let res = computer.output.pop().expect("There must be output");
            if res == 1 {
                ((x, y).try_into().unwrap(), GridState::Tractor)
            } else {
                ((x, y).try_into().unwrap(), GridState::Void)
            }
        })
        .collect();

    println!("{grid:#}");
    let mut left_side = Vec::with_capacity(50);
    let mut right_side = Vec::with_capacity(50);

    for y in 1..50 {
        let row = grid.get_row(y);
        let left = row
            .iter()
            .find(|(_, value)| matches!(value, GridState::Tractor))
            .map(|(pos, _)| *pos);
        left_side.extend(left.into_iter());
        let right = row
            .into_iter()
            .rfind(|(_, value)| matches!(value, GridState::Tractor))
            .map(|(pos, _)| pos);
        right_side.extend(right.into_iter());
    }

    let left = left_side
        .iter()
        .map(|coord| coord.y as f64 / coord.x as f64)
        .collect_vec();
    let right = right_side
        .iter()
        .map(|coord| coord.y as f64 / coord.x as f64)
        .collect_vec();
    let left_ = mode(&left).unwrap();
    // let left_ = left.iter().sum::<f64>() / left.len() as f64;
    let left = 1.0 / left_;
    let _right = 1.0 / mode(&right).unwrap();

    let left_yes: HashSet<_> = left_side.into_iter().collect();

    for x in 1..50 {
        let y = (x as f64 * left_).round() as i32;
        println!("{x}, {y}: {}", left_yes.contains(&(x, y).into()));
    }
    println!("===");
    for y in 1..50 {
        let x = (y as f64 * left).round() as i32;
        println!("{x}, {y}: {}", left_yes.contains(&(x, y).into()));
    }

    // for coord in left_side.iter() {
    //     let x = coord.y as f64 * left;
    //     let y = coord.x as f64 * left_;
    //     // println!(
    //     //     "real: x: {}, y: {}\ncalc: x: {}/{}, y: {}/{}\nequals: {} {}",
    //     //     coord.x,
    //     //     coord.y,
    //     //     x,
    //     //     x.round(),
    //     //     y,
    //     //     y.round(),
    //     //     coord.x == x.round() as i32,
    //     //     coord.y == y.round() as i32,
    //     // );
    //     println!(
    //         "{} {}",
    //         coord.x == x.round() as i32,
    //         coord.y == y.round() as i32,
    //     );
    // }

    Ok("".to_string())
}

inventory::submit!(crate::AoCDay::mew("2019", "19", main));
