use super::computer::Computer;

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    computer.run_to_completion().unwrap();
    Ok("".to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "5",
    func: main,
    example_func: None,
});
