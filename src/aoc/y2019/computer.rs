use std::collections::{BTreeMap, VecDeque};
use std::fmt::Display;
use std::str::FromStr;

use itertools::Itertools;

use crate::{AoCError, AoCResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<char> for ParameterMode {
    type Error = AoCError;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '0' => Ok(ParameterMode::Position),
            '1' => Ok(ParameterMode::Immediate),
            '2' => Ok(ParameterMode::Relative),
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
    AdjustRelative(ParameterMode),
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
            Self::Input(_) | Self::Output(_) | Self::AdjustRelative(_) => 1,
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
            "09" => Some(Opcode::AdjustRelative(flags[0].try_into().unwrap())),
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

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        (u32::from(value) as i128).into()
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
    relative_base: i128,
    pub input: VecDeque<i128>,
    pub output: Vec<i128>,
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
            relative_base: 0,
            input: VecDeque::new(),
            output: Vec::new(),
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

#[must_use]
pub enum RunState {
    NeedsInput,
    Finished,
}

impl Computer {
    pub fn run(&mut self) -> AoCResult<RunState> {
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
                    let target = self.get_target(&(self.pc + 3), target_mode)?;
                    self.set(target, Instruction(a + b));
                }
                Opcode::Mul(a_mode, b_mode, target_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    let b = self.get_with_mode(&(self.pc + 2), b_mode)?.to_value();
                    let target = self.get_target(&(self.pc + 3), target_mode)?;
                    self.set(target, Instruction(a * b));
                }
                Opcode::Input(target_mode) => {
                    let value = match self.input.pop_front() {
                        Some(value) => value,
                        None => return Ok(RunState::NeedsInput),
                    };
                    let target = self.get_target(&(self.pc + 1), target_mode)?;
                    self.set(target, value.into());
                }
                Opcode::Output(a_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    self.output.push(a);
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
                    let target = self.get_target(&(self.pc + 3), target_mode)?;
                    self.set(target, (if a < b { 1 } else { 0 }).into());
                }
                Opcode::Equal(a_mode, b_mode, target_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    let b = self.get_with_mode(&(self.pc + 2), b_mode)?.to_value();
                    let target = self.get_target(&(self.pc + 3), target_mode)?;
                    self.set(target, (if a == b { 1 } else { 0 }).into());
                }
                Opcode::AdjustRelative(a_mode) => {
                    let a = self.get_with_mode(&(self.pc + 1), a_mode)?.to_value();
                    self.relative_base = self
                        .relative_base
                        .checked_add(a)
                        .ok_or(AoCError::new("Relative base overflow"))?;
                }
                Opcode::End => return Ok(RunState::Finished),
            }
            self.pc += len + 1;
        }
    }

    pub fn run_to_completion(&mut self) -> AoCResult<()> {
        match self.run()? {
            RunState::Finished => Ok(()),
            RunState::NeedsInput => Err(AoCError::new("Program cannot complete without input!")),
        }
    }

    pub fn get(&self, key: &u64) -> Instruction {
        self.memory.get(key).copied().unwrap_or_default()
    }

    fn get_with_mode(&self, key: &u64, mode: ParameterMode) -> AoCResult<Instruction> {
        match mode {
            ParameterMode::Position => self.get_pos(key),
            ParameterMode::Immediate => Ok(self.get(key)),
            ParameterMode::Relative => self.get_relative(key),
        }
    }

    fn get_target(&self, key: &u64, mode: ParameterMode) -> AoCResult<u64> {
        match mode {
            ParameterMode::Position => self.get(key).to_memory_location(),
            ParameterMode::Immediate => unreachable!("this should never happen"),
            ParameterMode::Relative => self.resolve_relative_adress(key),
        }
    }

    pub fn get_pos(&self, key: &u64) -> AoCResult<Instruction> {
        Ok(self.get(&self.get(key).to_memory_location()?))
    }

    fn resolve_relative_adress(&self, key: &u64) -> AoCResult<u64> {
        let value = self.get(key);
        if self.relative_base == 0 {
            value.to_memory_location()
        } else {
            self.relative_base
                .checked_add(value.to_value())
                .ok_or(AoCError::new("Created a wildly too big relative address"))?
                .try_into()
                .map_err(|e| {
                    AoCError::new_with_cause("Created a moderately too big relative address", e)
                })
        }
    }

    pub fn get_relative(&self, key: &u64) -> AoCResult<Instruction> {
        Ok(self.get(&self.resolve_relative_adress(key)?))
    }

    pub fn set(&mut self, key: u64, value: Instruction) {
        self.memory.insert(key, value);
    }

    pub fn clear_output(&mut self) {
        self.output.clear();
    }

    pub fn get_ascii_output(&self) -> Option<String> {
        self.output
            .iter()
            .map(|i| {
                let v: u32 = (*i).try_into().ok()?;
                v.try_into()
                    .ok()
                    .and_then(|c: char| if c.is_ascii() { Some(c) } else { None })
            })
            .collect::<Option<String>>()
    }

    pub fn get_ascii_lossy(&self) -> String {
        self.output
            .iter()
            .filter_map(|i| {
                let v: u32 = (*i).try_into().ok()?;
                char::try_from(v).ok()
            })
            .collect()
    }

    pub fn add_ascii_input(&mut self, input: &str) {
        // Gonna trust you that it's ascii
        self.input
            .extend(input.chars().map(|c| u32::from(c) as i128))
    }
}
