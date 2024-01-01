use itertools::Itertools;

use super::computer::Computer;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    for line in data {
        let og_computer: Computer = line.parse().unwrap();
        for (noun, verb) in (0..=99).cartesian_product(0..=99) {
            let mut computer = og_computer.clone();
            computer.set(1, noun.into());
            computer.set(2, verb.into());
            computer.run_to_completion().unwrap();
            let res = computer.get(&0).to_value();
            println!("{noun} {verb} {res}");
            if res == 19690720 {
                return Ok(((100 * noun) + verb).to_string());
            }
        }
    }
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay::mew("2019", "2", main));
