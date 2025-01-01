// Copyright (c) 2024 Lexi Robinson
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

use crate::{partition_input, AoCError};

type WireValue = Option<u16>;

enum Input {
    Literal(u16),
    Wire(String),
}

impl FromStr for Input {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse() {
            Ok(Self::Literal(v))
        } else {
            Ok(Self::Wire(s.to_owned()))
        }
    }
}

impl Input {
    fn get_wire_name(&self) -> Option<String> {
        match self {
            Self::Wire(name) => Some(name.clone()),
            _ => None,
        }
    }

    fn get_value(&self, inputs: &mut Vec<u16>) -> WireValue {
        match self {
            Input::Literal(n) => Some(*n),
            Input::Wire(_) => inputs.pop(),
        }
    }
}

enum GateAction {
    And(Input, Input),
    Or(Input, Input),
    Xor(Input, Input),
}

impl FromStr for GateAction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref GATE_RE: Regex = Regex::new(r"^(.+) (AND|X?OR) (.+)$").unwrap();
        }

        if let Some(matches) = GATE_RE.captures(s) {
            let a: Input = matches[1].parse()?;
            let b: Input = matches[3].parse()?;
            Ok(match &matches[2] {
                "AND" => Self::And,
                "OR" => Self::Or,
                "XOR" => Self::Xor,
                wat => unreachable!("mystery gate {wat}"),
            }(a, b))
        } else {
            Err(AoCError::new(format!("Gate {s} does not match regex!")))
        }
    }
}

struct Gate {
    action: GateAction,
    output: String,
}

impl FromStr for Gate {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, output) = s
            .split_once(" -> ")
            .ok_or(AoCError::new(format!("Malformed gate definition {s}")))?;
        Ok(Self {
            action: action.parse()?,
            output: output.to_owned(),
        })
    }
}

impl Gate {
    fn get_inputs(&self) -> Vec<String> {
        match &self.action {
            GateAction::And(a, b) | GateAction::Or(a, b) | GateAction::Xor(a, b) => {
                vec![a.get_wire_name(), b.get_wire_name()]
            }
        }
        .into_iter()
        .flatten()
        .collect()
    }

    fn run(&self, mut inputs: Vec<u16>) -> WireValue {
        match &self.action {
            GateAction::And(a, b) => {
                let a = a.get_value(&mut inputs)?;
                let b = b.get_value(&mut inputs)?;
                Some(a & b)
            }
            GateAction::Or(a, b) => {
                let a = a.get_value(&mut inputs)?;
                let b = b.get_value(&mut inputs)?;
                Some(a | b)
            }
            GateAction::Xor(a, b) => {
                let a = a.get_value(&mut inputs)?;
                let b = b.get_value(&mut inputs)?;
                Some(a ^ b)
            }
        }
    }
}

fn drain_updates(
    gates: &[(Vec<String>, Gate)],
    wire_data: &mut HashMap<String, WireValue>,
    updates: &mut HashMap<String, WireValue>,
) {
    while !updates.is_empty() {
        let updated: HashSet<_> = updates.keys().cloned().collect();
        wire_data.extend(updates.drain());
        for (inputs, gate) in gates.iter() {
            if inputs.iter().any(|i| updated.contains(i)) {
                let res = gate.run(
                    inputs
                        .iter()
                        .flat_map(|name| wire_data.get(name).unwrap())
                        .copied()
                        .collect(),
                );
                if res.is_some() {
                    let existing = wire_data.get_mut(&gate.output).unwrap();
                    if &res != existing {
                        updates.insert(gate.output.clone(), res);
                    }
                }
            }
        }
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let (wires, gates) = partition_input(data);

    let gates: Vec<Gate> = gates.map(|line| line.parse()).try_collect()?;
    let mut wire_data: HashMap<String, WireValue> = gates
        .iter()
        .map(|gate| (gate.output.clone(), None))
        .chain(wires.map(|line| {
            let (gate, value) = line.split_once(": ").unwrap();
            (gate.to_owned(), value.parse().ok())
        }))
        .collect();
    let gates: Vec<_> = gates
        .into_iter()
        .map(|gate| (gate.get_inputs(), gate))
        .collect();

    let mut updates = HashMap::with_capacity(wire_data.len());
    updates.extend(
        wire_data
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(k, v)| (k.clone(), *v)),
    );

    drain_updates(&gates, &mut wire_data, &mut updates);

    let ret = wire_data
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_by_cached_key(|(k, _)| k.clone())
        .rev()
        .map(|(_, v)| v.expect("all z must be resolved"))
        .join("");
    let ret = u64::from_str_radix(&ret, 2).unwrap();
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "24",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: part_1
    }),
    // Part 2 was done non-programatically
    part_2: None,
});
