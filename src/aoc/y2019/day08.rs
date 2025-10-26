// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();
    let mut best_num_zero = usize::MAX;
    let mut ret = 0;
    for layer_iter in line.chars().chunks(WIDTH * HEIGHT).into_iter() {
        let layer: Vec<char> = layer_iter.collect();
        let num_zero = layer.iter().filter(|c| **c == '0').count();
        if num_zero < best_num_zero {
            ret = layer.iter().filter(|c| **c == '1').count()
                * layer.iter().filter(|c| **c == '2').count();
            best_num_zero = num_zero;
        }
    }
    Ok(ret.to_string())
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let line = data.next().unwrap();
    let mut base_layer: Option<Vec<char>> = None;
    for layer_iter in line.chars().rev().chunks(WIDTH * HEIGHT).into_iter() {
        let layer: Vec<char> = layer_iter.collect();
        match base_layer {
            None => base_layer = Some(layer),
            Some(base_data) => {
                base_layer = Some(
                    base_data
                        .into_iter()
                        .zip(layer)
                        .map(
                            |(base, incoming)| {
                                if incoming == '2' { base } else { incoming }
                            },
                        )
                        .collect(),
                );
            }
        }
    }
    for (i, char) in base_layer.unwrap().into_iter().rev().enumerate() {
        if i % WIDTH == 0 {
            println!();
        }
        print!(
            "{}",
            match char {
                '2' => '!',
                '1' => '#',
                _ => ' ',
            }
        );
    }
    println!();
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "8",
    part_1: crate::AoCPart {
        main: part_1,
        example: crate::no_example
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: crate::no_example
    }),
});
