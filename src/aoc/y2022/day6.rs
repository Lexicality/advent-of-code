use std::collections::{HashSet, VecDeque};
use std::str::Chars;

#[derive(Debug)]
struct SubRoutine(VecDeque<char>, HashSet<char>);

impl SubRoutine {
    fn new(chars: &mut Chars) -> SubRoutine {
        SubRoutine {
            0: chars.take(4).collect(),
            1: HashSet::with_capacity(4),
        }
    }

    fn bingo_bango(&mut self) -> bool {
        self.1.clear();
        for c in self.0.iter() {
            self.1.insert(*c);
        }
        return self.1.len() == 4;
    }

    fn rotate(&mut self, value: char) {
        self.0.pop_front().unwrap();
        self.0.push_back(value);
    }
}

pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    for line in data {
        let mut citer = line.chars();
        let mut sub = SubRoutine::new(&mut citer);
        println!("{:?}", sub);
        if sub.bingo_bango() {
            println!("4");
            continue;
        }
        let mut i = 5;
        for c in citer {
            sub.rotate(c);
            if sub.bingo_bango() {
                break;
            }
            i += 1
        }
        println!("{}", i);
    }
    "".to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "6",
    func: main,
});
