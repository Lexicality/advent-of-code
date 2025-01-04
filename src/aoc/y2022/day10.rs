// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

const MAGIC_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

enum Instruction {
    Noop,
    Tick,
    AddX(i32),
}

impl Instruction {
    fn new(inst: String) -> Vec<Instruction> {
        if inst == "noop" {
            return vec![Instruction::Noop];
        } else if let Some(stripped) = inst.strip_prefix("addx ") {
            return vec![
                Instruction::Tick,
                Instruction::AddX(stripped.parse().unwrap()),
            ];
        }
        panic!("Unknown instruction {}", inst);
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<8}",
            match self {
                Instruction::Noop => "Noop".to_string(),
                Instruction::Tick => "Tick".to_string(),
                Instruction::AddX(xadd) => format!("AddX {:+03}", xadd),
            }
        )
    }
}

struct Cpu {
    pc: usize,
    instructions: Vec<Instruction>,
    x: i64,
}

impl Cpu {
    fn new(data: &mut dyn Iterator<Item = String>) -> Cpu {
        Cpu {
            pc: 0,
            instructions: data.flat_map(Instruction::new).collect(),
            x: 1,
        }
    }

    fn cycle_part_1(&mut self) -> Option<i64> {
        let cycle = self.pc + 1;
        let instruction = &self.instructions[self.pc];
        let prex = self.x;
        print!(" {:>03} | {:>4} | {}", cycle, prex, instruction);
        if let Instruction::AddX(xadd) = instruction {
            self.x += *xadd as i64;
        }
        println!(" | {:>4}", self.x);
        self.pc = cycle;
        if MAGIC_CYCLES.contains(&cycle) {
            Some(prex * cycle as i64)
        } else {
            None
        }
    }

    fn cycle_part_2(&mut self) {
        let sprpos = self.x;
        let px = (self.pc % 40) as i64;
        if px == 0 {
            println!();
        }
        if px >= sprpos - 1 && px <= sprpos + 1 {
            print!("#");
        } else {
            print!(" ");
        }

        let instruction = &self.instructions[self.pc];
        if let Instruction::AddX(xadd) = instruction {
            self.x += *xadd as i64;
        }

        self.pc += 1;
    }

    fn done_part_1(&self) -> bool {
        // Don't care about anything after the last magic value
        self.pc >= MAGIC_CYCLES[5] || self.pc >= self.instructions.len()
    }

    fn done_part_2(&self) -> bool {
        self.pc >= self.instructions.len()
    }
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut cpu = Cpu::new(&mut data);
    println!(" {:>3} | {:^4} | {:^8} | {:4}", "pc", "x", "inst", "newx");
    let mut signals = 0;
    while !cpu.done_part_1() {
        let res = cpu.cycle_part_1();
        if let Some(signal) = res {
            println!("Got a signal: {}", signal);
            signals += signal;
        }
    }
    Ok(signals.to_string())
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut cpu = Cpu::new(&mut data);
    println!(" {:>3} | {:^4} | {:^8} | {:4}", "pc", "x", "inst", "newx");
    while !cpu.done_part_2() {
        cpu.cycle_part_2();
    }
    println!();
    Ok(String::new())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "10",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
