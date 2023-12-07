use itertools::Itertools;

const WIDTH: u32 = 25;
const HEIGHT: u32 = 6;

pub fn main(data: crate::DataIn) -> String {
    let line = data.next().unwrap();
    let mut best_num_zero = usize::MAX;
    let mut ret = 0;
    for layer_iter in line.chars().chunks((WIDTH * HEIGHT) as usize).into_iter() {
        let layer: Vec<char> = layer_iter.collect();
        let num_zero = layer.iter().filter(|c| **c == '0').count();
        if num_zero < best_num_zero {
            ret = layer.iter().filter(|c| **c == '1').count()
                * layer.iter().filter(|c| **c == '2').count();
            best_num_zero = num_zero;
        }
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "8",
    func: main,
});
