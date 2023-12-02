use itertools::Itertools;

use super::computer::Computer;

pub fn main(data: crate::DataIn) -> String {
    for line in data {
        let og_computer: Computer = line.parse().unwrap();
        for (noun, verb) in (0..=99).cartesian_product(0..=99) {
            let mut computer = og_computer.clone();
            computer.set(1, noun.into());
            computer.set(2, verb.into());
            computer.run().unwrap();
            let res = computer.get(&0).to_value();
            println!("{noun} {verb} {res}");
            if res == 19690720 {
                return ((100 * noun) + verb).to_string();
            }
        }
    }
    "".to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "2",
    func: main,
});
