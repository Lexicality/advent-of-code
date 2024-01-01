use super::computer::Computer;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    for line in data {
        let mut computer: Computer = line.parse().unwrap();
        computer.run_to_completion().unwrap();
        println!("---------");
    }
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay::mew("2019", "9", main));
