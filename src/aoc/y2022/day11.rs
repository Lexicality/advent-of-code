// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use itertools::Itertools;

type MonkeyID = usize;
type Item = u32;
type MonkeyYeet = (Item, MonkeyID);

fn get_data(data: crate::DataIter, text: &'static str) -> String {
    let line = data.next().unwrap();
    let line = line.trim();
    assert!(line.starts_with(text));
    line[text.len()..].to_string()
}

enum Op {
    Old,
    Value(u32),
}

impl Op {
    fn new(data: &str) -> Op {
        if data == "old" {
            Op::Old
        } else {
            Op::Value(data.parse().unwrap())
        }
    }

    fn get(&self, item: Item) -> u32 {
        match self {
            Op::Old => item,
            Op::Value(v) => *v,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Old => write!(f, "old"),
            Self::Value(v) => write!(f, "{}", v),
        }
    }
}

enum Action {
    Multiply,
    Add,
}

impl Action {
    fn new(data: &str) -> Action {
        match data {
            "+" => Action::Add,
            "*" => Action::Multiply,
            _ => panic!("Unknown action {}", data),
        }
    }

    fn act(&self, first: u32, second: u32) -> u32 {
        match self {
            Self::Add => first + second,
            Self::Multiply => first * second,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Add => "+",
                Action::Multiply => "*",
            }
        )
    }
}

struct Operation {
    first: Op,
    action: Action,
    second: Op,
}

impl Operation {
    fn new(data: crate::DataIter) -> Operation {
        let line = get_data(data, "Operation: new = ");
        let (first, action, second) = line.split(' ').collect_tuple().unwrap();
        Operation {
            first: Op::new(first),
            action: Action::new(action),
            second: Op::new(second),
        }
    }

    fn run(&self, item: Item) -> Item {
        self.action.act(self.first.get(item), self.second.get(item))
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "new = {} {} {}", self.first, self.action, self.second)
    }
}

struct Test {
    testor: u32,
    success: MonkeyID,
    failure: MonkeyID,
}

impl Test {
    fn new(data: crate::DataIter) -> Test {
        Test {
            testor: get_data(data, "Test: divisible by ").parse().unwrap(),
            success: get_data(data, "If true: throw to monkey ").parse().unwrap(),
            failure: get_data(data, "If false: throw to monkey ")
                .parse()
                .unwrap(),
        }
    }

    fn test(&self, item: Item) -> MonkeyID {
        if item % self.testor == 0 {
            self.success
        } else {
            self.failure
        }
    }
}

impl Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "i % {} == 0 ? {} : {}",
            self.testor, self.success, self.failure,
        )
    }
}

struct Monke {
    id: MonkeyID,
    items: Vec<Item>,
    operation: Operation,
    test: Test,
    inspections: u32,
}

impl Monke {
    fn new(data: crate::DataIter) -> Monke {
        let line = get_data(data, "Monkey ");
        Monke {
            id: line[..1].parse().unwrap(),
            items: get_data(data, "Starting items: ")
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect(),
            operation: Operation::new(data),
            test: Test::new(data),
            inspections: 0,
        }
    }

    fn round(&mut self) -> impl Iterator<Item = MonkeyYeet> + '_ {
        // println!("Monkey {}:", self.id);
        self.items.drain(..).map(|item| {
            // println!(" Inspects item {}", item);
            let item = self.operation.run(item);
            self.inspections += 1;
            // println!("  Modifies: {}", item);
            let item = item / 3;
            // println!("  Bored: {}", item);
            let target = self.test.test(item);
            // println!("  Yeets to: {}", target);
            (item, target)
        })
    }
}

impl Display for Monke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "Monkey {}:", self.id)?;
        // writeln!(f, "  Starting items: {}", self.items.iter().join(", "))?;
        // writeln!(f, "  Operation: {}", self.operation)?;
        // writeln!(f, "  Test: {}", self.test)?;
        // Ok(())
        write!(f, "Monkey {}: {}", self.id, self.items.iter().join(", "))
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut data = data.peekable();
    let mut monkeys = Vec::with_capacity(8);
    while data.peek().is_some() {
        monkeys.push(Monke::new(&mut data));
        if let Some(line) = data.next() {
            assert!(line.is_empty())
        } else {
            break;
        }
    }
    println!("Round 0:");
    for monkey in monkeys.iter() {
        println!(" {}", monkey);
    }

    for round in 1..21 {
        for i in 0..monkeys.len() {
            let yeets: Vec<_> = monkeys[i].round().collect();
            for (item, target) in yeets {
                monkeys.get_mut(target).unwrap().items.push(item);
            }
        }

        println!("Round {}:", round);
        for monkey in monkeys.iter() {
            println!(" {}", monkey);
        }
    }

    println!("Results:");
    for monkey in monkeys.iter() {
        println!(
            " Monkey {} inspected {} items",
            monkey.id, monkey.inspections
        );
    }

    let res: u32 = monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .product();

    Ok(res.to_string())
}

inventory::submit!(crate::AoCDay::mew("2022", "11", main));
