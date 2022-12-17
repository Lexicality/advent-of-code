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

    fn cycle(&mut self) {
        let sprpos = self.x;
        let px = (self.pc % 40) as i64;
        if px == 0 {
            println!("");
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
        return self.pc >= self.instructions.len();
    }
}

pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    let mut cpu = CPU::new(data);
    println!(" {:>3} | {:^4} | {:^8} | {:4}", "pc", "x", "inst", "newx");
    while !cpu.done() {
        cpu.cycle();
    }
    println!();
    return format!("");
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "10",
    func: main,
});
