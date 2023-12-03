use super::computer::Computer;

pub fn main(data: crate::DataIn) -> String {
    for line in data {
        let mut computer: Computer = line.parse().unwrap();
        computer.run().unwrap();
        println!("---------");
    }
    "".to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "9",
    func: main,
});
