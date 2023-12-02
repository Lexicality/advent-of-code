use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use itertools::Itertools;

use crate::AoCError;

pub type ComputerResult<T> = Result<T, AoCError>;

pub enum Opcode {
    Add,
    Mul,
    End,
}

impl Opcode {
    fn num_instructions(&self) -> u64 {
        match self {
            Self::Add => 3,
            Self::Mul => 3,
            Self::End => 0,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Instruction(i128);

impl Instruction {
    fn to_opcode(self) -> Option<Opcode> {
        match self.0 {
            1 => Some(Opcode::Add),
            2 => Some(Opcode::Mul),
            99 => Some(Opcode::End),
            _ => None,
        }
    }

    fn to_memory_location(self) -> ComputerResult<u64> {
        self.0
            .try_into()
            .map_err(|e| AoCError::new_with_cause(format!("Invalid memory address {}", self.0), e))
    }
}

impl From<i128> for Instruction {
    fn from(value: i128) -> Self {
        Instruction(value)
    }
}

impl FromStr for Instruction {
    type Err = AoCError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(Instruction)
            .map_err(|e| AoCError::new_with_cause("Invalid incode instruction", e))
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Computer {
    memory: BTreeMap<u64, Instruction>,
    pc: u64,
}

impl FromStr for Computer {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Computer {
            memory: s
                .split(',')
                .enumerate()
                .map(|(i, s)| Ok((i as u64, s.parse()?)))
                .collect::<Result<BTreeMap<u64, Instruction>, Self::Err>>()?,
            pc: 0,
        })
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.memory
                .iter()
                .map(|(k, i)| if *k == self.pc {
                    format!("*{i}*")
                } else {
                    format!("{i}")
                })
                .join(", ")
        )
    }
}

impl Computer {
    pub fn run(&mut self) -> ComputerResult<()> {
        loop {
            let instr = self.get(&self.pc);
            let opcode = instr
                .to_opcode()
                .ok_or_else(|| AoCError::new(format!("Invalid opcode {instr}")))?;
            let len = opcode.num_instructions();
            match opcode {
                Opcode::Add => {
                    let a = self.get_pos(&(self.pc + 1))?.0;
                    let b = self.get_pos(&(self.pc + 2))?.0;
                    let target = self.get(&(self.pc + 3));
                    self.set(target.to_memory_location()?, Instruction(a + b));
                }
                Opcode::Mul => {
                    let a = self.get_pos(&(self.pc + 1))?.0;
                    let b = self.get_pos(&(self.pc + 2))?.0;
                    let target = self.get(&(self.pc + 3));
                    self.set(target.to_memory_location()?, Instruction(a * b));
                }
                Opcode::End => return Ok(()),
            }
            self.pc += len + 1;
        }
    }

    pub fn get(&self, key: &u64) -> Instruction {
        self.memory.get(key).copied().unwrap_or_default()
    }

    pub fn get_pos(&self, pos_key: &u64) -> ComputerResult<Instruction> {
        Ok(self.get(&self.get(pos_key).to_memory_location()?))
    }

    pub fn set(&mut self, key: u64, value: Instruction) {
        self.memory.insert(key, value);
    }
}
