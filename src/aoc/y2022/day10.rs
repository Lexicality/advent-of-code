use std::fmt::Display;

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

    fn cycle(&mut self) {
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

    fn done(&self) -> bool {
        self.pc >= self.instructions.len()
    }
}

pub fn main(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut cpu = Cpu::new(&mut data);
    println!(" {:>3} | {:^4} | {:^8} | {:4}", "pc", "x", "inst", "newx");
    while !cpu.done() {
        cpu.cycle();
    }
    println!();
    Ok(String::new())
}

inventory::submit!(crate::AoCDay::mew("2022", "10", main));
