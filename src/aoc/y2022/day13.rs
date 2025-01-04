// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::EitherOrBoth as EoB;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::Display;

use serde_json::{json, Value};

enum Order {
    Right,
    Wrong,
}

fn actual_check_order(left: &Value, right: &Value) -> Option<Order> {
    match left {
        Value::Number(left) => match right {
            Value::Number(right) => {
                if left == right {
                    return None;
                }
                let left = left.as_u64().unwrap();
                let right = right.as_u64().unwrap();
                match left.cmp(&right) {
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Order::Wrong),
                    Ordering::Less => Some(Order::Right),
                }
            }
            Value::Array(_) => actual_check_order(&json!([left]), right),
            _ => panic!("Unexpected value!"),
        },
        Value::Array(left) => {
            let rvec: Vec<_>;
            let right = match right {
                Value::Number(_) => {
                    rvec = vec![right.clone()];
                    &rvec
                }
                Value::Array(right) => right,
                _ => panic!("Unexpected value!"),
            };
            let iter = left.iter().zip_longest(right);
            for item in iter {
                match item {
                    EoB::Left(_) => return Some(Order::Wrong),
                    EoB::Right(_) => return Some(Order::Right),
                    EoB::Both(left, right) => {
                        let res = actual_check_order(left, right);
                        if res.is_some() {
                            return res;
                        }
                    }
                }
            }
            None
        }
        _ => panic!("Unexpected value!"),
    }
}

fn check_order(left: Value, right: Value) -> Order {
    actual_check_order(&left, &right).unwrap()
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut index = 0;
    let mut ret = 0;
    loop {
        index += 1;
        let v1: Value = serde_json::from_str(&data.next().unwrap()).unwrap();
        let v2: Value = serde_json::from_str(&data.next().unwrap()).unwrap();
        let order = check_order(v1, v2);
        if let Order::Right = order {
            ret += index;
        }

        if data.next().is_none() {
            break;
        }
    }
    Ok(ret.to_string())
}

struct Sigh<'a>(&'a Vec<Value>);

impl Display for Sigh<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0 {
            line.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut signal = Vec::with_capacity(data.size_hint().0 + 2);
    let first_marker = json!([[2]]);
    let second_marker = json!([[6]]);
    signal.push(first_marker.clone());
    signal.push(second_marker.clone());
    for line in data {
        if line.is_empty() {
            continue;
        }
        signal.push(serde_json::from_str(&line).unwrap());
    }
    println!("{}", Sigh(&signal));
    signal.sort_unstable_by(
        |left, right| match actual_check_order(left, right).unwrap() {
            Order::Right => Ordering::Less,
            Order::Wrong => Ordering::Greater,
        },
    );
    println!("{}", Sigh(&signal));
    let fpos = signal
        .iter()
        .find_position(|v| v == &&first_marker)
        .map(|d| d.0)
        .expect("Can't find first marker")
        + 1;
    let spos = signal
        .iter()
        .find_position(|v| v == &&second_marker)
        .map(|d| d.0)
        .expect("Can't find second marker")
        + 1;
    Ok((fpos * spos).to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "13",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
