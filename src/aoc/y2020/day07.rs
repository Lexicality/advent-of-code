// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, AoCResult};

type BagID = String;

#[derive(Debug)]
struct BagRule {
    count: u32,
    bag: BagID,
}

struct Bag {
    id: BagID,
    contains: Vec<BagRule>,
    contained_by: HashSet<BagID>,
}

impl FromStr for Bag {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BAG_RE: Regex = Regex::new(r"^(\w+ \w+) bags contain (.+).").unwrap();
            static ref RULE_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
            //static ref
        }
        let matches = BAG_RE
            .captures(s)
            .ok_or_else(AoCError::new_from_regex(s, &BAG_RE))?;

        let rules = RULE_RE
            .captures_iter(&matches[2])
            .map(|matches| -> AoCResult<BagRule> {
                Ok(BagRule {
                    count: matches[1].parse()?,
                    bag: matches[2].to_owned(),
                })
            })
            .try_collect()?;

        Ok(Bag {
            id: matches[1].to_owned(),
            contains: rules,
            contained_by: Default::default(),
        })
    }
}

fn baggo(data: crate::DataIn) -> crate::AoCResult<HashMap<BagID, Bag>> {
    let mut bags: HashMap<BagID, Bag> = data
        .map(|line| line.parse())
        .map_ok(|bag: Bag| (bag.id.clone(), bag))
        .try_collect()?;

    let mut containeriser: HashMap<BagID, HashSet<BagID>> = HashMap::with_capacity(bags.len());

    for bag in bags.values() {
        bag.contains.iter().for_each(|rule| {
            containeriser
                .entry(rule.bag.clone())
                .or_default()
                .insert(bag.id.clone());
        });
    }

    for (id, contained_by) in containeriser.into_iter() {
        bags.entry(id)
            .and_modify(|bag| bag.contained_by = contained_by);
    }

    Ok(bags)
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let bags = baggo(data)?;

    let my_bag = &bags["shiny gold"];

    let mut to_check: Vec<&BagID> = my_bag.contained_by.iter().collect();
    let mut seen: HashSet<&BagID> = to_check.iter().copied().collect();
    let mut ret = 0;
    while let Some(bagid) = to_check.pop() {
        log::debug!("{bagid}");
        ret += 1;
        to_check.extend(
            bags[bagid]
                .contained_by
                .iter()
                .filter(|bag_id| seen.insert(*bag_id)),
        );
    }

    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let bags = baggo(data)?;

    let my_bag = &bags["shiny gold"];

    let mut to_check: Vec<&BagRule> = my_bag.contains.iter().collect();
    let mut ret = 0;
    while let Some(rule) = to_check.pop() {
        ret += rule.count;

        for _ in 0..rule.count {
            to_check.extend(bags[&rule.bag].contains.iter());
        }
    }

    Ok(ret.to_string())
}
inventory::submit!(crate::AoCDay {
    year: "2020",
    day: "7",
    part_1: crate::AoCPart {
        main: part_1,
        example: |data| crate::partitioned_example(data, part_1)
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: |data| crate::partitioned_example(data, part_2)
    })
});
