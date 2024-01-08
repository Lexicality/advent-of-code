use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::AoCError;

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
    RShift(Input, Input),
    LShift(Input, Input),
    Not(Input),
    Const(Input),
}

impl FromStr for GateAction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref NOT_RE: Regex = Regex::new(r"^NOT (.+)$").unwrap();
            static ref GATE_RE: Regex = Regex::new(r"^(.+) (AND|OR|RSHIFT|LSHIFT) (.+)$").unwrap();
        }

        if let Some(matches) = NOT_RE.captures(s) {
            Ok(Self::Not(matches[1].parse()?))
        } else if let Some(matches) = GATE_RE.captures(s) {
            let a: Input = matches[1].parse()?;
            let b: Input = matches[3].parse()?;
            Ok(match &matches[2] {
                "AND" => Self::And,
                "OR" => Self::Or,
                "RSHIFT" => Self::RShift,
                "LSHIFT" => Self::LShift,
                wat => unreachable!("mystery gate {wat}"),
            }(a, b))
        } else if let Ok(const_input) = s.parse() {
            Ok(Self::Const(const_input))
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
            GateAction::Const(i) | GateAction::Not(i) => vec![i.get_wire_name()],
            GateAction::And(a, b)
            | GateAction::Or(a, b)
            | GateAction::RShift(a, b)
            | GateAction::LShift(a, b) => vec![a.get_wire_name(), b.get_wire_name()],
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
            GateAction::RShift(a, b) => {
                let a = a.get_value(&mut inputs)?;
                let b = b.get_value(&mut inputs)?;
                Some(a >> b)
            }
            GateAction::LShift(a, b) => {
                let a = a.get_value(&mut inputs)?;
                let b = b.get_value(&mut inputs)?;
                Some(a << b)
            }
            GateAction::Not(a) => {
                let a = a.get_value(&mut inputs)?;
                Some(!a)
            }
            GateAction::Const(a) => a.get_value(&mut inputs),
        }
    }
}

fn init_updates(gates: &[(Vec<String>, Gate)]) -> impl Iterator<Item = (String, WireValue)> + '_ {
    gates.iter().flat_map(|(_, gate)| {
        let init = gate.run(Vec::new());
        if init.is_some() {
            Some((gate.output.clone(), init))
        } else {
            None
        }
    })
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

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let gates: Vec<Gate> = data.map(|line| line.parse()).try_collect()?;
    let mut wire_data: HashMap<String, WireValue> = gates
        .iter()
        .map(|gate| (gate.output.clone(), None))
        .collect();
    let gates: Vec<_> = gates
        .into_iter()
        .map(|gate| (gate.get_inputs(), gate))
        .collect();
    let mut updates = HashMap::with_capacity(wire_data.len());
    updates.extend(init_updates(&gates));

    drain_updates(&gates, &mut wire_data, &mut updates);

    let ret = wire_data
        .get("a")
        .expect("wire a must exist")
        .expect("a must have a value");
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2015", "7", main));
