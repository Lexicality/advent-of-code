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
        } else if inst.starts_with("addx ") {
            return vec![
                Instruction::Tick,
                Instruction::AddX(inst[5..].parse().unwrap()),
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

struct CPU {
    pc: usize,
    instructions: Vec<Instruction>,
    x: i64,
}

impl CPU {
    fn new(data: &mut dyn Iterator<Item = String>) -> CPU {
        CPU {
            pc: 0,
            instructions: data.flat_map(Instruction::new).collect(),
            x: 1,
        }
    }

    fn cycle(&mut self) -> Option<i64> {
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
            return Some(prex * cycle as i64);
        } else {
            return None;
        }
    }

    fn done(&self) -> bool {
        // Don't care about anything after the last magic value
        return self.pc >= MAGIC_CYCLES[5] || self.pc >= self.instructions.len();
    }
}

pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    let mut cpu = CPU::new(data);
    println!(" {:>3} | {:^4} | {:^8} | {:4}", "pc", "x", "inst", "newx");
    let mut signals = 0;
    while !cpu.done() {
        let res = cpu.cycle();
        if let Some(signal) = res {
            println!("Got a signal: {}", signal);
            signals += signal;
        }
    }
    return format!("{}", signals);
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "10",
    func: main,
});
