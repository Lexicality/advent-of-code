// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashSet;

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::AoCError;

lazy_static! {
    static ref REQUIRED_FIELDS: HashSet<&'static str> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .collect();
    static ref VALID_EYES: HashSet<&'static str> =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .into_iter()
            .collect();
}

fn parse_pports(data: crate::DataIn) -> crate::AoCResult<Vec<Vec<(String, String)>>> {
    data.batching(|iter| {
        let line = iter.take_while(|line| !line.is_empty()).join(" ");
        if line.is_empty() {
            return None;
        }
        Some(
            line.split_whitespace()
                .map(|field| {
                    field
                        .split_once(":")
                        .ok_or_else(|| AoCError::new(format!("Field {field} doesn't have a :")))
                        .map(|(a, b)| (a.to_owned(), b.to_owned()))
                })
                .try_collect(),
        )
    })
    .try_collect()
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let passports = parse_pports(data)?;
    let ret = passports
        .into_iter()
        .inspect(|pport| log::debug!("passport {pport:?}"))
        .filter(|pport| {
            pport
                .iter()
                .map(|(key, _)| key.as_str())
                .collect::<HashSet<_>>()
                .is_superset(&REQUIRED_FIELDS)
        })
        .count();

    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let passports = parse_pports(data)?;
    let ret = passports
        .into_iter()
        .inspect(|pport| log::debug!("passport {pport:?}"))
        .filter(|pport| {
            pport
                .iter()
                .map(|(key, _)| key.as_str())
                .collect::<HashSet<_>>()
                .is_superset(&REQUIRED_FIELDS)
        })
        .filter(|pport| {
            pport.iter().all(|(key, value)| {
                let value = value.as_str();
                let res = match key.as_str() {
                    "byr" => value.len() == 4 && ("1920"..="2002").contains(&value),
                    "iyr" => value.len() == 4 && ("2010"..="2020").contains(&value),
                    "eyr" => value.len() == 4 && ("2020"..="2030").contains(&value),
                    "hgt" if value.ends_with("cm") => {
                        value.len() == 5 && ("150cm"..="193cm").contains(&value)
                    }
                    "hgt" if value.ends_with("in") => {
                        value.len() == 4 && ("59in"..="76in").contains(&value)
                    }
                    "hgt" => false,
                    "hcl" => {
                        value.len() == 7
                            && value.starts_with('#')
                            && value.chars().skip(1).all(|c| {
                                c.is_ascii_hexdigit()
                                    && (c.is_ascii_lowercase() || c.is_ascii_digit())
                            })
                    }
                    "ecl" => VALID_EYES.contains(value),
                    "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
                    _ => true,
                };
                log::debug!("{key}: {value} is valid? {res}");
                res
            })
        })
        .count();

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2020",
    day: "4",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});
