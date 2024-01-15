use itertools::Itertools;

use super::computer::Computer;

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let base_computer: Computer = data.next().unwrap().parse().unwrap();

    let ret = (0..50)
        .cartesian_product(0..50)
        .map(|(x, y)| {
            let mut computer = base_computer.clone();
            computer.input.push_back(x);
            computer.input.push_back(y);
            computer
                .run_to_completion()
                .expect("The computer must work");
            computer.output.pop().expect("There must be output")
        })
        .sum::<i64>();

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2019", "19", main));
