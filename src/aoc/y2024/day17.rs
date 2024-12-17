// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

use crate::{AoCError, AoCResult};

type Integer = i64;
type ProgamValue = u8;

#[derive(Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bcx,
    Out,
    Bdv,
    Cdv,
}

impl From<ProgamValue> for Opcode {
    fn from(value: ProgamValue) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bcx,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!("Program values are 3 bits"),
        }
    }
}

struct Computer {
    reg_a: Integer,
    reg_b: Integer,
    reg_c: Integer,
    program: Vec<ProgamValue>,
    pc: usize,
}

fn parse_register(line: String) -> AoCResult<Integer> {
    let (_, value) = line
        .split_once(": ")
        .ok_or(AoCError::new(format!("colon missing from '{line}'!")))?;
    value.parse().map_err(AoCError::new_from_parseerror)
}

impl Computer {
    fn new(mut data: crate::DataIn) -> AoCResult<Self> {
        let (line_reg_a, line_reg_b, line_reg_c, _, line_program) = data
            .next_tuple()
            .ok_or(AoCError::new("Not enough lines for the computer!"))?;

        Ok(Self {
            reg_a: parse_register(line_reg_a)?,
            reg_b: parse_register(line_reg_b)?,
            reg_c: parse_register(line_reg_c)?,
            program: {
                let (_, value) = line_program.split_once(": ").ok_or(AoCError::new(format!(
                    "colon missing from '{line_program}'!"
                )))?;
                value
                    .split(',')
                    .map(str::parse)
                    .try_collect()
                    .map_err(AoCError::new_from_parseerror)?
            },
            pc: 0,
        })
    }

    fn literal_operand(&self) -> Integer {
        self.program[self.pc + 1].into()
    }

    fn combo_operand(&self) -> Integer {
        match self.program[self.pc + 1] {
            v @ 0..=3 => v.into(),
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            v => panic!("Invalid combo operand {v}!"),
        }
    }

    fn computate(mut self) -> Vec<Integer> {
        let mut ret = vec![];
        loop {
            if self.pc > self.program.len() - 1 {
                return ret;
            };
            match self.program[self.pc].into() {
                opcode @ (Opcode::Adv | Opcode::Bdv | Opcode::Cdv) => {
                    let ret = self.reg_a
                        / (2_i64.pow(self.combo_operand().try_into().unwrap_or_else(|e| {
                            panic!("Invalid combo operand for {opcode:?}: {e}")
                        })));
                    match opcode {
                        Opcode::Adv => self.reg_a = ret,
                        Opcode::Bdv => self.reg_b = ret,
                        Opcode::Cdv => self.reg_c = ret,
                        _ => unreachable!(),
                    }
                }
                Opcode::Bxl => {
                    self.reg_b ^= self.literal_operand();
                }
                Opcode::Bst => {
                    self.reg_b = self.combo_operand() % 8;
                }
                Opcode::Jnz => {
                    if self.reg_a != 0 {
                        let target = self.literal_operand();
                        assert!(target >= 0, "cannot jump to a negative target {target}!!");
                        self.pc = target.try_into().unwrap();
                        continue;
                    }
                }
                Opcode::Bcx => {
                    // "For legacy reasons, this instruction reads an operand
                    // but ignores it." - very suspicious
                    // we don't need to read anything because reading has no
                    // effect, but something to take a note of
                    self.reg_b ^= self.reg_c;
                }
                Opcode::Out => {
                    ret.push(self.combo_operand() % 8);
                }
            }
            self.pc += 2;
        }
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let computer = Computer::new(data)?;
    let ret = computer.computate().into_iter().join(",");
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "17", main));
