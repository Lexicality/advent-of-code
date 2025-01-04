// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::AoCError;

#[derive(Debug)]
struct Node {
    name: String,
    parents: Vec<String>,
    children: Vec<String>,
}

impl FromStr for Node {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, children) = s
            .split_once(':')
            .ok_or_else(|| AoCError::new("Expected a : but didn't find one"))?;
        Ok(Self {
            name: name.to_owned(),
            parents: Vec::new(),
            children: children.trim().split(' ').map(|s| s.to_owned()).collect(),
        })
    }
}

impl Node {
    fn mew(name: String, parent: String) -> Self {
        Self {
            name,
            parents: vec![parent.to_owned()],
            children: Vec::new(),
        }
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let nodes: Vec<Node> = data.map(|line| line.parse()).try_collect()?;
    let mut nodes: HashMap<String, Node> = nodes
        .into_iter()
        .map(|node| (node.name.clone(), node))
        .collect();

    let parenting = nodes
        .values()
        .flat_map(|node| {
            node.children
                .iter()
                .cloned()
                .map(|child| (node.name.clone(), child))
                .collect_vec()
        })
        .collect_vec();

    for (parent, child) in parenting.into_iter() {
        match nodes.get_mut(&child) {
            Some(node) => {
                node.parents.push(parent);
            }
            None => {
                nodes.insert(child.clone(), Node::mew(child, parent));
            }
        }
    }

    println!("{nodes:#?}");

    let ret = 0;
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "25",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None,
});
