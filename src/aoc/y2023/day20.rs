use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    str::FromStr,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::AoCError;

#[derive(Debug, Clone, Copy)]
enum SignalState {
    Low,
    High,
}

impl SignalState {
    fn toggle(self) -> Self {
        match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }
}

impl Display for SignalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::High => "high",
            Self::Low => "low",
        }
        .fmt(f)
    }
}

enum ModuleType {
    Broadcaster,
    FlipFlop(SignalState),
    Conjunction(HashMap<String, SignalState>),
    Dummy,
}

struct Module {
    module_type: ModuleType,
    name: String,
    targets: Vec<String>,
}

impl FromStr for Module {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(broadcaster|[%&]\w+) -> (.+)$").unwrap();
        }

        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Line {s} does not match regex!")))?;

        let (module_type, name) = {
            match &matches[1] {
                name @ "broadcaster" => (ModuleType::Broadcaster, name.to_owned()),
                name => (
                    match name.chars().next().unwrap() {
                        '%' => ModuleType::FlipFlop(SignalState::Low),
                        '&' => ModuleType::Conjunction(HashMap::new()),
                        c => unreachable!("very unexpected name starter {c}"),
                    },
                    name.chars().skip(1).collect(),
                ),
            }
        };

        let targets = matches[2].split(',').map(|s| s.trim().to_owned()).collect();

        Ok(Module {
            module_type,
            name,
            targets,
        })
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} -> {}",
            match self.module_type {
                ModuleType::Broadcaster | ModuleType::Dummy => "",
                ModuleType::FlipFlop(_) => "%",
                ModuleType::Conjunction(_) => "&",
            },
            self.name,
            self.targets.iter().join(", ")
        )
    }
}

impl Module {
    fn add_input(&mut self, input: &str) {
        if let ModuleType::Conjunction(ref mut inputs) = &mut self.module_type {
            inputs.insert(input.to_owned(), SignalState::Low);
        }
    }

    fn receive_input(&mut self, from: &str, sig: SignalState) -> Option<SignalState> {
        match self.module_type {
            ModuleType::Broadcaster => Some(sig),
            ModuleType::FlipFlop(ref mut current_state) => match sig {
                SignalState::High => None,
                SignalState::Low => {
                    *current_state = current_state.toggle();
                    Some(*current_state)
                }
            },
            ModuleType::Conjunction(ref mut inputs) => {
                *(inputs.get_mut(from).expect("Inputs must be predefned")) = sig;
                if inputs.values().all(|v| matches!(v, SignalState::High)) {
                    Some(SignalState::Low)
                } else {
                    Some(SignalState::High)
                }
            }
            ModuleType::Dummy => None,
        }
    }
}

fn push_the_button(modules: &mut HashMap<String, Module>) -> (u64, u64) {
    let mut lows = 0;
    let mut highs = 0;
    let mut signals = VecDeque::new();
    signals.push_back((
        "broadcaster".to_owned(),
        "button".to_owned(),
        SignalState::Low,
    ));
    while let Some((to, from, signal)) = signals.pop_front() {
        // println!("{from} -{signal}-> {to}");
        match signal {
            SignalState::High => highs += 1,
            SignalState::Low => lows += 1,
        }
        let module = modules.get_mut(&to).expect("destination must exist");
        let response = module.receive_input(&from, signal);
        if let Some(sig) = response {
            signals.extend(
                module
                    .targets
                    .iter()
                    .cloned()
                    .map(|to| (to, module.name.clone(), sig)),
            )
        }
    }
    (lows, highs)
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut modules: HashMap<String, Module> = data
        .map(|line| line.parse())
        .map_ok(|module: Module| (module.name.clone(), module))
        .try_collect()?;

    // bah
    let targets: HashMap<String, Vec<String>> = modules
        .iter()
        .map(|(name, m)| (name.clone(), m.targets.clone()))
        .collect();

    for (name, targets) in targets.into_iter() {
        for target in targets.into_iter() {
            match modules.get_mut(&target) {
                Some(m) => m.add_input(&name),
                None => {
                    println!("Inserting dummy module {target}!");
                    modules.insert(
                        target.clone(),
                        Module {
                            module_type: ModuleType::Dummy,
                            name: target,
                            targets: Vec::new(),
                        },
                    );
                }
            }
        }
    }
    modules
        .get_mut("broadcaster")
        .expect("must have a broadcaster")
        .add_input("button");

    let (lows, highs) = (0..1000)
        .map(|_| push_the_button(&mut modules))
        // .inspect(|(low, high)| println!("== {low} low signals, {high} high signals =="))
        .reduce(|(al, ah), (l, h)| (al + l, ah + h))
        .unwrap();
    Ok((lows * highs).to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "20",
    func: main,
    example_func: None,
});
