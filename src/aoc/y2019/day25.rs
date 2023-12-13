use text_io::read;

use super::computer::{Computer, RunState};

pub fn main(data: crate::DataIn) -> String {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    while let RunState::NeedsInput = computer.run().unwrap() {
        print!("{}", computer.get_ascii_output().unwrap());
        computer.clear_output();
        print!("> ");
        let line: String = read!("{}\n");
        let line = line.trim().to_owned() + "\n";
        computer.add_ascii_input(&line);
    }
    "".to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "25",
    func: main,
    example_func: None,
});
