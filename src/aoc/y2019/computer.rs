use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

use itertools::Itertools;
use text_io::read;

use crate::{AoCError, AoCResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl TryFrom<char> for ParameterMode {
    type Error = AoCError;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '0' => Ok(ParameterMode::Position),
            '1' => Ok(ParameterMode::Immediate),
            _ => Err(AoCError::new(format!("Unknown parameter mode {s}"))),
        }
    }
}

#[derive(Debug)]
pub enum Opcode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equal(ParameterMode, ParameterMode, ParameterMode),
    End,
}

impl Opcode {
    fn num_instructions(&self) -> u64 {
        match self {
            Self::Add(_, _, _)
            | Self::Mul(_, _, _)
            | Self::LessThan(_, _, _)
            | Self::Equal(_, _, _) => 3,
            Self::JumpIfTrue(_, _) | Self::JumpIfFalse(_, _) => 2,
            Self::Input(_) | Self::Output(_) => 1,
            Self::End => 0,
        }
    }

    fn validate(&self) -> AoCResult<()> {
        match self {
            Self::Add(_, _, ParameterMode::Immediate)
            | Self::Mul(_, _, ParameterMode::Immediate)
            | Self::LessThan(_, _, ParameterMode::Immediate)
            | Self::Equal(_, _, ParameterMode::Immediate)
            | Self::Input(ParameterMode::Immediate) => Err(AoCError::new(
                "Cannot use immediate mode addressing for a target parameter",
            )),
            _ => Ok(()),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Instruction(i128);

impl Instruction {
    pub fn to_opcode(self) -> Option<Opcode> {
        let value = self.0;
        if !(0..=99999).contains(&value) {
            return None;
        }
        let value = &format!("{value:05}");
        let (flags, opcode) = value.split_at(3);
        let flags: Vec<char> = flags.chars().rev().collect();
        // println!("opcode: {opcode}");
        // println!("flags: {flags:?}");

        match opcode {
            "01" => Some(Opcode::Add(
                flags[0].try_into().unwrap(),
                flags[1].try_into().unwrap(),
                flags[2].try_into().unwrap(),
            )),
            "02" => Some(Opcode::Mul(
                flags[0].try_into().unwrap(),
                flags[1].try_into().unwrap(),
                flags[2].try_into().unwrap(),
            )),
            "03" => Some(Opcode::Input(flags[0].try_into().unwrap())),
            "04" => Some(Opcode::Output(flags[0].try_into().unwrap())),
            "05" => Some(Opcode::JumpIfTrue(
                flags[0].try_into().unwrap(),
                flags[1].try_into().unwrap(),
            )),
            "06" => Some(Opcode::JumpIfFalse(
                flags[0].try_into().unwrap(),
                flags[1].try_into().unwrap(),
            )),
            "07" => Some(Opcode::LessThan(
                flags[0].try_into().unwrap(),
                flags[1].try_into().unwrap(),
                flags[2].try_into().unwrap(),
            )),
            "08" => Some(Opcode::Equal(
                flags[0].try_into().unwrap(),
                flags[1].try_into().unwrap(),
                flags[2].try_into().unwrap(),
            )),

            "99" => Some(Opcode::End),
            _ => None,
        }
        .map(|opcode| {
            opcode.validate().unwrap();
            opcode
        })
    }

    pub fn to_memory_location(self) -> AoCResult<u64> {
        self.0
            .try_into()
            .map_err(|e| AoCError::new_with_cause(format!("Invalid memory address {}", self.0), e))
    }

    pub fn to_value(self) -> i128 {
        self.0
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

#[derive(Clone)]
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
    pub fn run(&mut self) -> AoCResult<()> {
        loop {
            let instr = self.get(&self.pc);
            let opcode = instr
                .to_opcode()
                .ok_or_else(|| AoCError::new(format!("Invalid opcode {instr}")))?;
            let len = opcode.num_instructions();
            match opcode {
                Opcode::Add(a_mode, b_mode, target_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    let b = self.get_with_mode(&(self.pc + 2), b_mode)?.to_value();
                    let target = self.get_target(&(self.pc + 3), target_mode);
                    self.set(target.to_memory_location()?, Instruction(a + b));
                }
                Opcode::Mul(a_mode, b_mode, target_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    let b = self.get_with_mode(&(self.pc + 2), b_mode)?.to_value();
                    let target = self.get_target(&(self.pc + 3), target_mode);
                    self.set(target.to_memory_location()?, Instruction(a * b));
                }
                Opcode::Input(target_mode) => {
                    let target = self.get_target(&(self.pc + 1), target_mode);
                    print!("Please enter a number: ");
                    let value: i128 = read!("{}\n");
                    self.set(target.to_memory_location()?, value.into());
                }
                Opcode::Output(a_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    println!("{}", a);
                }
                Opcode::JumpIfTrue(a_mode, b_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    let b = self
                        .get_with_mode(&(self.pc + 2), b_mode)?
                        .to_memory_location()?;
                    if a != 0 {
                        self.pc = b;
                        continue;
                    }
                }
                Opcode::JumpIfFalse(a_mode, b_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    let b = self
                        .get_with_mode(&(self.pc + 2), b_mode)?
                        .to_memory_location()?;
                    if a == 0 {
                        self.pc = b;
                        continue;
                    }
                }
                Opcode::LessThan(a_mode, b_mode, target_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    let b = self.get_with_mode(&(self.pc + 2), b_mode)?.to_value();
                    let target = self.get_target(&(self.pc + 3), target_mode);
                    self.set(
                        target.to_memory_location()?,
                        (if a < b { 1 } else { 0 }).into(),
                    );
                }
                Opcode::Equal(a_mode, b_mode, target_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    let b = self.get_with_mode(&(self.pc + 2), b_mode)?.to_value();
                    let target = self.get_target(&(self.pc + 3), target_mode);
                    self.set(
                        target.to_memory_location()?,
                        (if a == b { 1 } else { 0 }).into(),
                    );
                }
                Opcode::End => return Ok(()),
            }
            self.pc += len + 1;
        }
    }

    pub fn get(&self, key: &u64) -> Instruction {
        self.memory.get(key).copied().unwrap_or_default()
    }

    fn get_with_mode(&self, key: &u64, mode: ParameterMode) -> AoCResult<Instruction> {
        match mode {
            ParameterMode::Position => self.get_pos(key),
            ParameterMode::Immediate => Ok(self.get(key)),
        }
    }

    fn get_target(&self, key: &u64, mode: ParameterMode) -> Instruction {
        match mode {
            ParameterMode::Position => self.get(key),
            ParameterMode::Immediate => unreachable!("this should never happen"),
        }
    }

    pub fn get_pos(&self, pos_key: &u64) -> AoCResult<Instruction> {
        Ok(self.get(&self.get(pos_key).to_memory_location()?))
    }

    pub fn set(&mut self, key: u64, value: Instruction) {
        self.memory.insert(key, value);
    }
}
