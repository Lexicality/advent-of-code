use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    open: bool,
    tunnels: Vec<String>,
}

impl Valve {
    pub fn parse(line: String) -> Valve {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels lead to valves (.+)$")
                    .unwrap();
        }

        let matches = RE.captures(&line).expect("Line should match regex?");
        Valve {
            name: matches[1].to_owned(),
            flow_rate: matches[2].parse().unwrap(),
            open: false,
            tunnels: matches[2]
                .split(',')
                .map(|name| name.trim().to_owned())
                .collect(),
        }
    }

    pub fn potential_flow(&self) -> u32 {
        if !self.open {
            self.flow_rate
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct Volcano {
    valves: HashMap<String, Valve>,
    pressure_per_second: u32,
    pressure_released: u32,
}

#[allow(dead_code)]
impl Volcano {
    pub fn new(data: crate::DataIn) -> Volcano {
        Volcano {
            valves: data
                .map(Valve::parse)
                .map(|v| (v.name.clone(), v))
                .collect(),
            pressure_per_second: 0,
            pressure_released: 0,
        }
    }

    pub fn get(&self, id: &str) -> &Valve {
        self.valves.get(id).unwrap()
    }

    pub fn get_mut(&mut self, id: &str) -> &mut Valve {
        self.valves.get_mut(id).unwrap()
    }

    pub fn get_open(&self) -> Vec<&Valve> {
        self.valves.values().filter(|v| v.open).collect()
    }

    pub fn tree(&self, start: &str) -> Vec<(String, u32, Vec<String>)> {
        let mut seen: HashSet<String> = HashSet::with_capacity(self.valves.len());
        seen.insert(start.to_string());
        // This would be so unbelievably more elegant as a generator
        let mut ret: Vec<(String, u32, Vec<String>)> = Vec::with_capacity(self.valves.len());
        let mut queue: VecDeque<_> = self
            .get(start)
            .tunnels
            .iter()
            .map(|t| (1, t, vec![]))
            .collect();

        while let Some((cost, id, path)) = queue.pop_front() {
            let valve = self.get(id);
            let potential = valve.potential_flow();
            ret.push((id.to_owned(), potential.saturating_sub(cost), path.clone()));
            seen.insert(id.clone());
            valve
                .tunnels
                .iter()
                .filter(|t| !seen.contains(*t))
                .for_each(|t| {
                    let mut path = path.clone();
                    path.push(id.to_owned());
                    queue.push_back((cost + 1, t, path));
                });
        }

        ret
    }

    pub fn get_target(&self, pos: &str) -> Option<String> {
        let mut tree = self.tree(pos);
        tree.sort_by_key(|v| v.1);
        let best = &tree[0];
        let current = self.get(pos).potential_flow();
        if best.1 == 0 && current == 0 {
            None
        } else if current >= best.1 {
            Some(pos.to_owned())
        } else {
            best.2.first().or(Some(&best.0)).map(|a| a.to_owned())
        }
    }

    pub fn open(&mut self, pos: &str) {
        let valve = self.get_mut(pos);
        valve.open = true;
        self.pressure_per_second += valve.flow_rate;
    }
}

impl Display for Volcano {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let open = self.get_open();
        match open.len() {
            0 => write!(f, "No valves are open"),
            1 => write!(
                f,
                "Valve {} is open, releasing {} pressure",
                open[0].name, self.pressure_per_second
            ),
            _ => write!(
                f,
                "Valves {} are open, releasing {} pressure",
                open.iter().map(|v| v.name.clone()).join(", "),
                self.pressure_per_second
            ),
        }
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut volcano = Volcano::new(data);
    let mut current_valve = "AA".to_owned();

    for minute in 1..=30 {
        println!("== Minute {minute} ==");
        println!("{volcano}");
        println!("You are at {}", current_valve);
        let Some(target) = volcano.get_target(&current_valve) else {
            break;
        };
        if target == current_valve {
            println!("You open valve {}", target);
            volcano.open(&current_valve);
        } else {
            println!("You move to valve {}", target);
            current_valve = target;
        }
        volcano.pressure_released += volcano.pressure_per_second;
    }

    Ok(volcano.pressure_released.to_string())
}

inventory::submit!(crate::AoCDay::mew("2022", "16", main));
