use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::AoCError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Category {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Category {
    fn get_from_part(self, part: &Part) -> u32 {
        match self {
            Self::ExtremelyCool => part.x,
            Self::Musical => part.m,
            Self::Aerodynamic => part.a,
            Self::Shiny => part.s,
        }
    }
}

impl TryFrom<char> for Category {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Self::ExtremelyCool),
            'm' => Ok(Self::Musical),
            'a' => Ok(Self::Aerodynamic),
            's' => Ok(Self::Shiny),
            _ => Err(AoCError::new_from_char(value)),
        }
    }
}

#[derive(Debug)]
enum Condition {
    CategoryGreater(Category, u32),
    CategoryLesser(Category, u32),
    Unconditional,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Target {
    Accept,
    Reject,
    Workflow(String),
}

impl FromStr for Target {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Reject),
            "A" => Ok(Self::Accept),
            _ => Ok(Self::Workflow(s.to_owned())),
        }
    }
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    target: Target,
}

impl Rule {
    fn check_part<'a>(&'a self, part: &Part) -> Option<&'a Target> {
        (match self.condition {
            Condition::CategoryGreater(category, value) => category.get_from_part(part) > value,
            Condition::CategoryLesser(category, value) => category.get_from_part(part) < value,
            Condition::Unconditional => true,
        })
        .then_some(&self.target)
    }
}

impl FromStr for Rule {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([xmas])(>|<)(\d+):(\w+)$").unwrap();
        }

        let matches = RE.captures(s);

        match matches {
            Some(matches) => {
                let category = matches[1].chars().next().unwrap().try_into()?;
                let value = matches[3].parse().map_err(AoCError::new_from_parseerror)?;
                let condition = if &matches[2] == ">" {
                    Condition::CategoryGreater(category, value)
                } else {
                    Condition::CategoryLesser(category, value)
                };
                let target = matches[4].parse()?;
                Ok(Self { condition, target })
            }
            None => Ok(Self {
                condition: Condition::Unconditional,
                target: s.parse()?,
            }),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    unconditional: Option<Target>,
}

impl Workflow {
    fn check_part(&self, part: &Part) -> &Target {
        if let Some(target) = self.unconditional.as_ref() {
            return target;
        }
        self.rules
            .iter()
            .find_map(|r| r.check_part(part))
            .expect("At least one rule must always match")
    }
}

impl FromStr for Workflow {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+)\{(.+)\}$").unwrap();
        }
        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Line {s} does not match regex!")))?;

        let name = matches[1].to_owned();
        let rules: Vec<Rule> = matches[2].split(',').map(|s| s.parse()).try_collect()?;

        assert!(
            !rules.is_empty(),
            "Must have at least one rule per workflow"
        );
        assert!(
            rules
                .iter()
                .any(|r| matches!(r.condition, Condition::Unconditional)),
            "Every workflow must have at least one unconditional step"
        );
        let unconditional = rules
            .iter()
            .try_fold(rules[0].target.clone(), |target, rule| {
                if target == rule.target {
                    Some(target)
                } else {
                    None
                }
            });

        Ok(Self {
            name,
            rules,
            unconditional,
        })
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn to_rating(self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
        }
        let matches = RE
            .captures(s)
            .ok_or_else(|| AoCError::new(format!("Line {s} does not match regex!")))?;

        Ok(Self {
            x: matches[1].parse().map_err(AoCError::new_from_parseerror)?,
            m: matches[2].parse().map_err(AoCError::new_from_parseerror)?,
            a: matches[3].parse().map_err(AoCError::new_from_parseerror)?,
            s: matches[4].parse().map_err(AoCError::new_from_parseerror)?,
        })
    }
}

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let workflows: HashMap<String, Workflow> = data
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| l.parse())
        .map_ok(|w: Workflow| (w.name.clone(), w))
        .try_collect()?;
    let parts: Vec<Part> = data.map(|l| l.parse()).try_collect()?;

    let ret = parts
        .into_iter()
        .filter(|part| {
            let mut wf = workflows.get("in").unwrap();
            loop {
                match wf.check_part(part) {
                    Target::Accept => return true,
                    Target::Reject => return false,
                    Target::Workflow(name) => {
                        wf = workflows
                            .get(name)
                            .unwrap_or_else(|| panic!("No workflow named {name}!"))
                    }
                }
            }
        })
        .map(|part| part.to_rating())
        .sum::<u32>();

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "19",
    func: main,
    example_func: None,
});
