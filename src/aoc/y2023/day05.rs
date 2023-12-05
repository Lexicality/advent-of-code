use std::ops::Range;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, AoCResult};

#[allow(dead_code)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

impl Mapping {
    fn parse(data: crate::DataIn) -> AoCResult<Self> {
        lazy_static! {
            static ref NAME_RE: Regex = Regex::new(r"^(.+)-(.+) map:$").unwrap();
        }
        let name_line = data.next().ok_or(AoCError::new("Name line is missing"))?;
        let matches = NAME_RE
            .captures(&name_line)
            .ok_or_else(|| AoCError::new(format!("Name line {name_line} doesn't match regex")))?;
        let mut ret = Mapping {
            from: matches[1].to_owned(),
            to: matches[2].to_owned(),
            ranges: Vec::new(),
        };
        for line in data {
            if line.is_empty() {
                break;
            }

            let (dest, source, count) = line
                .splitn(3, ' ')
                .map(|num| num.parse().map_err(AoCError::new_from_parseerror))
                .collect::<Result<Vec<u64>, _>>()?
                .drain(..)
                .collect_tuple()
                .ok_or_else(|| AoCError::new(format!("Bad range line {line}")))?;
            ret.ranges
                .push((source..(source + count), dest..(dest + count)));
        }

        Ok(ret)
    }

    fn map(&self, value: u64) -> u64 {
        if let Some((from, to)) = self.ranges.iter().find(|(start, _)| start.contains(&value)) {
            (value - from.start) + to.start
        } else {
            value
        }
    }
}

pub fn main(data: crate::DataIn) -> String {
    let mut data = data.peekable();
    let seeds = data.next().unwrap();
    let mut seeds = seeds.split(' ');
    assert_eq!(seeds.next(), Some("seeds:"));
    let seeds: Vec<u64> = seeds
        .map(|num| num.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    assert!(data.next().unwrap().is_empty());
    // these are in the correct order so I'm not going to bother with name lookups for now
    let mut mappings: Vec<Mapping> = Vec::with_capacity(7);
    while data.peek().is_some() {
        mappings.push(Mapping::parse(&mut data).unwrap());
    }

    let ret = seeds
        .iter()
        .map(|seed| mappings.iter().fold(*seed, |val, mapper| mapper.map(val)))
        .min()
        .unwrap();

    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "5",
    func: main,
});
