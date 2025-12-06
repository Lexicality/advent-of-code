// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::str::FromStr;

use itertools::Itertools;

use crate::{AoCError, AoCResult};

type Num = u64;

#[derive(Debug, Clone, Copy)]
enum Input {
    Number(Num),
    Add,
    Mul,
}

impl Input {
    fn num(self) -> AoCResult<Num> {
        match self {
            Self::Number(num) => Ok(num),
            _ => Err(AoCError::new("Expected a number got {self:?} instead")),
        }
    }

    fn operate(&self, acc: Num, action: Self) -> AoCResult<Num> {
        let num = self.num()?;
        match action {
            Self::Mul => Ok(acc * num),
            Self::Add => Ok(acc + num),
            _ => Err(AoCError::new("Unexpected action {action:?}")),
        }
    }
}

impl FromStr for Input {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            num => Self::Number(num.parse()?),
        })
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let rows: Vec<Vec<Input>> = data
        .map(|line| {
            line.split_whitespace()
                .map(|atom| atom.parse())
                .try_collect()
        })
        .try_collect()?;

    // quick safety check
    let row_len = rows[0].len();
    if !rows.iter().all(|row| row.len() == row_len) {
        return Err(AoCError::new("Not all rows are the same length!"));
    }

    // annoyingly there's no multizip that works on arbitrary length chains so I guess we're going
    // to do this
    let ret: Num = (0..row_len)
        .map(|i| rows.iter().map(|row| row[i]).collect_vec())
        .map(|problem| {
            log::debug!("got a problem yeah? {problem:?}");
            let mut problem = problem.into_iter();
            let action = problem.next_back();
            match action {
                Some(action @ Input::Add) => {
                    problem.try_fold(0, |acc, num| num.operate(acc, action))
                }
                Some(action @ Input::Mul) => {
                    problem.try_fold(1, |acc, num| num.operate(acc, action))
                }
                Some(Input::Number(num)) => {
                    Err(AoCError::new(format!("action is a number? {num}")))
                }
                None => unreachable!("problem is empty"),
            }
        })
        .inspect(|v| log::debug!("Got a result yeah {v:?}"))
        .try_fold(0, |acc, value| -> AoCResult<Num> { Ok(acc + (value?)) })?;

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "6",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None
});
