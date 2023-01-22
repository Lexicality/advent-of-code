use itertools::EitherOrBoth as EoB;
use itertools::Itertools;
use std::cmp::Ordering;

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
            let right = match right {
                Value::Number(right) => json!([right]),
                Value::Array(_) => right.clone(),
                _ => panic!("Unexpected value!"),
            };
            let right = right.as_array().unwrap();
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

fn check_order(left: Value, right: Value) -> Order {
    actual_check_order(&left, &right).unwrap()
}

pub fn main(data: crate::DataIn) -> String {
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
    format!("{}", ret)
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "13",
    func: main,
});
