use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::Direction;

pub fn main(data: crate::DataIn) -> String {
    let mut instructions = data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::East,
            'R' => Direction::West,
            _ => unreachable!(),
        })
        .collect_vec()
        .into_iter()
        .cycle();

    assert_eq!(data.next(), Some("".to_owned()));

    let nodes: HashMap<String, (String, String)> = data
        .map(|line| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(...) = \((...), (...)\)$").unwrap();
            }
            let matches = RE
                .captures(&line)
                .unwrap_or_else(|| panic!("{line} didn't match regex"));

            (
                matches[1].to_owned(),
                (matches[2].to_owned(), matches[3].to_owned()),
            )
        })
        .collect();

    println!("{nodes:#?}");

    let mut ret = 0;
    let mut pos = "AAA";
    while pos != "ZZZ" {
        ret += 1;
        let node = nodes
            .get(pos)
            .unwrap_or_else(|| panic!("don't have a node for {pos}!!!"));
        print!("{pos}, {node:?} ");
        pos = match instructions.next().unwrap() {
            Direction::East => &node.0,
            Direction::West => &node.1,
            _ => unreachable!(),
        };
        println!("=> {pos}");
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "8",
    func: main,
});
