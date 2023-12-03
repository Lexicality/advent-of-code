use super::computer::Computer;

pub fn main(data: crate::DataIn) -> String {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    computer.run().unwrap();
    "".to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "5",
    func: main,
});