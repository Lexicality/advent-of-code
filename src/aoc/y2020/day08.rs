// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

use crate::AoCError;

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Opcode {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opcode, arg) = s
            .split_once(' ')
            .ok_or_else(|| AoCError::new(format!("Expected {s} to have a space in it!")))?;
        let arg = arg.parse()?;
        Ok(match opcode {
            "acc" => Self::Acc(arg),
            "jmp" => Self::Jmp(arg),
            "nop" => Self::Nop(arg),
            _ => return Err(AoCError::new(format!("Unexpected opcode {opcode}"))),
        })
    }
}

impl Opcode {
    fn swapulate(self) -> Self {
        match self {
            Self::Acc(_) => self,
            Self::Jmp(i) => Self::Nop(i),
            Self::Nop(i) => Self::Jmp(i),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    InfiniteLoop,
    Error,
    Terminated,
}

fn computate(opcodes: &[Opcode]) -> (Outcome, i32) {
    let mut pc = 0;
    let mut acc = 0;
    let max = opcodes.len();
    let mut visited_instructions = HashSet::with_capacity(max);
    log::debug!("Got {max} opcodes!");
    while pc < max {
        if !visited_instructions.insert(pc) {
            log::debug!("Got an infinite loop at {pc}!");
            return (Outcome::InfiniteLoop, acc);
        }
        let opcode = opcodes[pc];
        log::debug!("pc is at {pc}, acc is {acc}, opcode is {opcode:?}");
        match opcode {
            Opcode::Acc(amt) => acc += amt,
            Opcode::Jmp(offset) => {
                if let Some(new_pc) = pc.checked_add_signed(offset as isize) {
                    log::debug!("Jumping to {new_pc}!");
                    pc = new_pc;
                    continue;
                } else {
                    log::error!("Opcode {opcode:?} set pc below zero!");
                    return (Outcome::Error, acc);
                }
            }
            Opcode::Nop(_) => (),
        }
        pc += 1;
    }
    log::debug!("Program terminated with pc at {pc} and acc at {acc}!");
    (Outcome::Terminated, acc)
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let opcodes: Vec<Opcode> = data.map(|line| line.parse()).try_collect()?;
    let (outcome, acc) = computate(&opcodes);
    assert!(matches!(outcome, Outcome::InfiniteLoop));
    let ret = acc;
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let opcodes: Vec<Opcode> = data.map(|line| line.parse()).try_collect()?;

    let ret = (0..opcodes.len())
        .map(|i| {
            let mut modified = opcodes.clone();
            modified[i] = modified[i].swapulate();
            computate(&modified)
        })
        .find_map(|(outcome, acc)| match outcome {
            Outcome::InfiniteLoop => None,
            _ => Some(acc),
        })
        .ok_or(AoCError::new("no modifications worked!"))?;

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2020",
    day: "8",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});
