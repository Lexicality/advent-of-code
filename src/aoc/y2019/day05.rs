use super::computer::Computer;

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    computer.run_to_completion().unwrap();
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay::mew("2019", "5", main));
