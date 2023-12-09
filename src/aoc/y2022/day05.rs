use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Yard {
    stacks: HashMap<u32, Vec<char>>,
    count: u32,
}

// fully cheating on perf by looking at input
const NUM_STACKS: usize = 9;
const NUM_BOXES: usize = 56;

impl Yard {
    fn new(data: &mut dyn Iterator<Item = String>) -> Yard {
        let mut yard = Yard {
            // fully cheating here by looking at the input
            stacks: HashMap::with_capacity(NUM_STACKS),
            count: 0,
        };
        // fully cheating here
        for line in data {
            if line.trim().is_empty() {
                // Cave Johnson; we're done here
                return yard;
            }
            assert!(line.is_ascii(), "non-ascii?? {}", line);
            if line.trim().starts_with('[') {
                // it's a stack line
                let mut boxes_in_row: &str = &line;
                let mut stack_i: u32 = 0;
                while !boxes_in_row.is_empty() {
                    stack_i += 1;
                    let current_box;
                    if boxes_in_row.len() == 3 {
                        current_box = boxes_in_row;
                        boxes_in_row = "";
                    } else {
                        (current_box, boxes_in_row) = boxes_in_row.split_at(4);
                    }
                    if current_box.trim().is_empty() {
                        continue;
                    }
                    let current_box = current_box.chars().nth(1).unwrap();
                    yard.stacks
                        .entry(stack_i)
                        .or_insert_with(|| Vec::with_capacity(NUM_BOXES));
                    yard.stacks
                        .get_mut(&stack_i)
                        .unwrap()
                        .insert(0, current_box);
                }
            } else {
                // it's (hopefully?) the stack list?
                yard.count = line
                    .trim()
                    .split("   ")
                    .map(|c| c.parse::<u32>().unwrap())
                    .max()
                    .unwrap();
            }
        }
        panic!("We should never get here")
    }

    fn perform_move(&mut self, op: &Move) {
        println!("Moving {} from {} to {}", op.count, op.start, op.end);
        let boxes: Vec<_> = (0..op.count)
            .map(|_| {
                let moving_box = self.stacks.get_mut(&op.start).unwrap().pop().unwrap();
                println!(" Moving {} from {} to {}", moving_box, op.start, op.end);
                moving_box
            })
            .collect();
        self.stacks
            .get_mut(&op.end)
            .unwrap()
            .extend(boxes.iter().rev());
    }

    /// destructive because it's late and I want this damn thing done
    fn get_top(&mut self) -> String {
        (1..self.count + 1)
            .map(|i| self.stacks.get_mut(&i).unwrap().pop().unwrap())
            .join("")
    }
}

impl Display for Yard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth = self.stacks.values().map(|v| v.len()).max().unwrap();
        for depth in (0..depth).rev() {
            for stack in 1..self.count + 1 {
                write!(f, "{} ", self.stacks[&stack].get(depth).unwrap_or(&' '))?;
            }
            writeln!(f)?;
        }
        for stack in 1..self.count + 1 {
            write!(f, "{} ", stack)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Move {
    count: u32,
    start: u32,
    end: u32,
}

impl Move {
    fn new(line: String) -> Move {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let matches = RE.captures(&line).unwrap();
        Move {
            count: matches[1].parse().unwrap(),
            start: matches[2].parse().unwrap(),
            end: matches[3].parse().unwrap(),
        }
    }
}

pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    let mut yard = Yard::new(data);
    println!("{}", yard);
    for op in data.map(Move::new) {
        yard.perform_move(&op);
        println!("{}", yard);
    }

    yard.get_top()
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "5",
    func: main,
    example_func: None,
});
