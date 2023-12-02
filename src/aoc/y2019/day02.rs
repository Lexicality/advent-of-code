use super::computer::Computer;

pub fn main(data: crate::DataIn) -> String {
    for line in data {
        let mut computer: Computer = line.parse().unwrap();
        // println!("{computer}");
        computer.set(1, 12.into());
        computer.set(2, 2.into());
        computer.run().unwrap();
        println!("{}", computer.get(&0))
    }
    "".to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "2",
    func: main,
});
