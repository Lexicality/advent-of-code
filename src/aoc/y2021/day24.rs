#![allow(dead_code, unused_variables, unused_mut)]
// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

use crate::AoCError;

type ComputerValue = i64;

#[derive(Debug, Clone, Copy)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl TryFrom<&str> for Register {
    type Error = AoCError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            _ => return Err(AoCError::new(format!("Unknown register value '{s}'"))),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Placeholder {
    Register(Register),
    Value(ComputerValue),
}

impl TryFrom<&str> for Placeholder {
    type Error = AoCError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(value) = s.parse::<ComputerValue>() {
            Ok(Self::Value(value))
        } else {
            Ok(Self::Register(s.try_into()?))
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Input(Register),
    Add(Register, Placeholder),
    Multiply(Register, Placeholder),
    Divide(Register, Placeholder),
    Modulo(Register, Placeholder),
    Equals(Register, Placeholder),
}

impl TryFrom<&str> for Instruction {
    type Error = AoCError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (opcode, extra) = s
            .split_once(' ')
            .ok_or(AoCError::new(format!("Invalid instruction input {s}!")))?;
        let a;
        let b;
        match opcode {
            "add" | "mul" | "div" | "mod" | "eql" => {
                (a, b) = extra
                    .split_once(' ')
                    .ok_or(AoCError::new(format!("Invalid {opcode} input {extra}!")))?;
            }
            _ => {
                a = extra;
                b = "";
            }
        };
        Ok(match opcode {
            "inp" => Self::Input(a.try_into()?),
            "add" => Self::Add(a.try_into()?, b.try_into()?),
            "mul" => Self::Multiply(a.try_into()?, b.try_into()?),
            "div" => Self::Divide(a.try_into()?, b.try_into()?),
            "mod" => Self::Modulo(a.try_into()?, b.try_into()?),
            "eql" => Self::Equals(a.try_into()?, b.try_into()?),
            _ => return Err(AoCError::new(format!("Unknown opcode value '{opcode}'"))),
        })
    }
}

struct Computer {
    w: ComputerValue,
    y: ComputerValue,
    x: ComputerValue,
    z: ComputerValue,
}

impl Computer {
    fn new() -> Self {
        Self {
            w: 0,
            y: 0,
            x: 0,
            z: 0,
        }
    }
    fn compute(&mut self, instructions: &[Instruction], mut input: Vec<ComputerValue>) {
        input.reverse();

        for instruction in instructions {
            match instruction {
                Instruction::Input(register) => {
                    self.set(register, input.pop().expect("must have input available"))
                }
                Instruction::Add(register, placeholder) => {
                    self.set(register, self.get_r(register) + self.get_p(placeholder))
                }
                Instruction::Multiply(register, placeholder) => {
                    self.set(register, self.get_r(register) * self.get_p(placeholder))
                }
                Instruction::Divide(register, placeholder) => {
                    self.set(register, self.get_r(register) / self.get_p(placeholder))
                }
                Instruction::Modulo(register, placeholder) => {
                    self.set(register, self.get_r(register) % self.get_p(placeholder))
                }
                Instruction::Equals(register, placeholder) => self.set(register, {
                    if self.get_r(register) == self.get_p(placeholder) {
                        1
                    } else {
                        0
                    }
                }),
            }
        }
    }

    fn get_p(&self, v: &Placeholder) -> ComputerValue {
        match v {
            Placeholder::Register(register) => self.get_r(register),
            Placeholder::Value(v) => *v,
        }
    }

    fn get_r(&self, v: &Register) -> ComputerValue {
        match v {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn set(&mut self, v: &Register, value: ComputerValue) {
        match v {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value,
        }
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let instructions: Vec<Instruction> = data.map(|line| line.as_str().try_into()).try_collect()?;

    for num in (1111_11111_11111_u64..=9999_99999_99999).rev() {
        let numstr = num.to_string();
        if numstr.contains('0') {
            continue;
        }
        let mut computer = Computer::new();
        computer.compute(
            &instructions,
            numstr
                .chars()
                .map(|c| c.to_digit(10).unwrap() as ComputerValue)
                .collect(),
        );
        if computer.z == 0 {
            return Ok(num.to_string());
        }
    }

    Err(AoCError::new("Nothing computed??"))
}

pub fn part_1_example(data: crate::DataIn) -> crate::AoCResult<String> {
    let instructions: Vec<Instruction> = data.map(|line| line.as_str().try_into()).try_collect()?;
    let mut computer = Computer::new();

    computer.compute(&instructions, vec![10, 30]);

    Ok(format!(
        "w: {} x: {}, y: {}, z: {}",
        computer.w, computer.x, computer.y, computer.z
    ))
}

inventory::submit!(crate::AoCDay {
    year: "2021",
    day: "24",
    part_1: Some(crate::AoCPart {
        main: part_1,
        example: |data| crate::partitioned_example(data, part_1_example)
    }),
    part_2: None,
});
