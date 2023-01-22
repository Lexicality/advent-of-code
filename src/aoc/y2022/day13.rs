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
            return None;
        }
        _ => panic!("Unexpected value!"),
    }
}

struct Sigh<'a>(&'a Vec<Value>);

impl<'a> Display for Sigh<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0 {
            line.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn main(data: crate::DataIn) -> String {
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
    format!("{}", fpos * spos)
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "13",
    func: main,
});
