use std::ops::Range;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, AoCResult};

#[derive(Debug)]
enum WrangleState {
    Disjoint,
    MatchesWhole(Range<u64>),
    MatchesPart(Range<u64>, Vec<Range<u64>>),
}

#[allow(clippy::single_range_in_vec_init)]
fn range_wrangle(input: &Range<u64>, range: &Range<u64>) -> WrangleState {
    let contains_start = range.contains(&input.start);
    let contains_end = range.contains(&(input.end - 1));
    if input.is_empty() {
        unreachable!("input {input:?} is empty");
    }
    match (contains_start, contains_end) {
        (true, true) => WrangleState::MatchesWhole(input.clone()),
        (false, true) => {
            WrangleState::MatchesPart(range.start..input.end, vec![input.start..range.start])
        }
        (true, false) => {
            WrangleState::MatchesPart(input.start..range.end, vec![range.end..input.end])
        }
        (false, false) => {
            if input.start < range.start && input.end > range.end {
                WrangleState::MatchesPart(
                    range.clone(),
                    vec![input.start..range.start, range.end..input.end],
                )
            } else {
                WrangleState::Disjoint
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

impl Mapping {
    fn parse(data: crate::DataIter) -> AoCResult<Self> {
        lazy_static! {
            static ref NAME_RE: Regex = Regex::new(r"^(.+)-to-(.+) map:$").unwrap();
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
                .into_iter()
                .collect_tuple()
                .ok_or_else(|| AoCError::new(format!("Bad range line {line}")))?;
            ret.ranges
                .push((source..(source + count), dest..(dest + count)));
        }
        // Sort for easier munging
        ret.ranges.sort_by_key(|(start, _)| start.start);
        let mut fillers = Vec::with_capacity(3); // There's only one disjoint per category

        let start = ret.ranges.first().unwrap().0.start;
        fillers.push(0..start);
        let end = ret.ranges.last().unwrap().0.end;
        fillers.push(end..u64::MAX);

        for (a, b) in ret.ranges.iter().map(|(start, _)| start).tuple_windows() {
            if a.end != b.start {
                fillers.push(a.end..b.start);
            }
        }
        for filler in fillers {
            ret.ranges.push((filler.clone(), filler));
        }
        // Sort again just for my own sanity
        ret.ranges.sort_by_key(|(start, _)| start.start);

        Ok(ret)
    }

    fn map_ranges(&self, mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut output = Vec::with_capacity(ranges.len());
        while let Some(input) = ranges.pop() {
            for (start, end) in self.ranges.iter() {
                let input = match range_wrangle(&input, start) {
                    WrangleState::MatchesWhole(input) => input,
                    WrangleState::Disjoint => continue,
                    WrangleState::MatchesPart(input, remainder) => {
                        ranges.extend(remainder);
                        input
                    }
                };
                let offset = input.start - start.start;
                let size = input.end - input.start;
                let result = (end.start + offset)..(end.start + offset + size);
                if result.is_empty() {
                    panic!("wat input: {input:?} start: {start:?} end: {end:?}\n offset: {offset} size: {size}\n result: {result:?}");
                }
                output.push(result);
                break;
            }
        }
        output
    }
}

pub fn get_seeds(line: &str) -> Vec<Range<u64>> {
    let mut seeds_iter = line.split(' ');
    assert_eq!(seeds_iter.next(), Some("seeds:"));
    let seeds_tmp = seeds_iter
        .map(|num| num.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(seeds_tmp.len() % 2, 0, "non-even number of seeds??");
    let mut ret = Vec::with_capacity(seeds_tmp.len() / 2);
    let mut seeds_iter = seeds_tmp.into_iter();
    while let Some((start, count)) = seeds_iter.next_tuple() {
        ret.push(start..(start + count));
    }
    ret
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut data = data.peekable();
    let seeds = get_seeds(&data.next().unwrap());
    assert!(data.next().unwrap().is_empty());
    // these are in the correct order so I'm not going to bother with name lookups for now
    let mut mappings: Vec<Mapping> = Vec::with_capacity(7);
    while data.peek().is_some() {
        mappings.push(Mapping::parse(&mut data).unwrap());
    }

    let ret = seeds
        .into_iter()
        .map(|seed| {
            mappings
                .iter()
                .fold(vec![seed], |val, mapper| mapper.map_ranges(val))
                .into_iter()
                .map(|range| range.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2023", "5", main));
