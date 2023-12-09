use itertools::Itertools;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn main(data: crate::DataIn) -> String {
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
    "".to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "8",
    func: main,
    example_func: None,
});
