// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

use crate::{AoCError, AoCResult};

#[derive(Debug)]
struct Node {
    child_nodes: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn new(numbers: &mut dyn Iterator<Item = u32>) -> AoCResult<Self> {
        let node_count = numbers.next().ok_or(AoCError::new("missing node count"))?;
        let metadata_count = numbers
            .next()
            .ok_or(AoCError::new("missing metadata count"))
            .and_then(|num| {
                if num != 0 {
                    Ok(num)
                } else {
                    Err(AoCError::new("metadata count is zero"))
                }
            })?;
        Ok(Self {
            child_nodes: (0..node_count).map(|_| Node::new(numbers)).try_collect()?,
            metadata: numbers.take(metadata_count as usize).collect(),
        })
    }

    fn part_1_sum(&self) -> u32 {
        self.metadata.iter().sum::<u32>()
            + self
                .child_nodes
                .iter()
                .map(|node| node.part_1_sum())
                .sum::<u32>()
    }

    fn get_node_value(&self, index: u32) -> u32 {
        self.child_nodes
            .get(index as usize - 1)
            .map(|node| node.part_2_sum())
            .unwrap_or_default()
    }

    fn part_2_sum(&self) -> u32 {
        if self.child_nodes.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|index| self.get_node_value(*index))
                .sum()
        }
    }
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let numbers: Vec<u32> = data
        .next()
        .unwrap()
        .split(' ')
        .map(|s| s.parse())
        .try_collect()
        .map_err(AoCError::new_from_parseerror)?;
    let root_node = Node::new(&mut numbers.into_iter())?;
    let ret = root_node.part_1_sum();
    Ok(ret.to_string())
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let numbers: Vec<u32> = data
        .next()
        .unwrap()
        .split(' ')
        .map(|s| s.parse())
        .try_collect()
        .map_err(AoCError::new_from_parseerror)?;
    let root_node = Node::new(&mut numbers.into_iter())?;
    let ret = root_node.part_2_sum();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2018",
    day: "8",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
