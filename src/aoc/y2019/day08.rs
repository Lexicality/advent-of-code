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

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
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
                                if incoming == '2' {
                                    base
                                } else {
                                    incoming
                                }
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

inventory::submit!(crate::AoCDay::mew("2019", "8", main));
