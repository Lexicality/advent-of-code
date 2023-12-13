use itertools::Itertools;

use crate::aoc::y2019::computer::{Computer, RunState};

const NUM_MACHINES: usize = 50;

pub fn main(data: crate::DataIn) -> String {
    let base_code: Computer = data.next().unwrap().parse().unwrap();

    let mut packets_to_go: Vec<Vec<i128>> = (0..NUM_MACHINES).map(|_| Vec::new()).collect();
    let mut computetrs: Vec<Computer> = (0..NUM_MACHINES)
        .map(|addr| {
            let mut computer = base_code.clone();
            computer.input.push_back(addr as i128);
            computer
        })
        .collect();

    loop {
        for (addr, computer) in computetrs.iter_mut().enumerate() {
            match computer.run().unwrap() {
                RunState::NeedsInput => {
                    let packets = packets_to_go.get_mut(addr).unwrap();
                    if packets.is_empty() {
                        computer.input.push_back(-1);
                    } else {
                        computer.input.extend(packets.drain(..));
                    }
                }
                RunState::Finished => panic!("Computer {addr} finished?!"),
            }
            assert_eq!(computer.output.len() % 3, 0);
            for (target_addr, x, y) in computer.output.drain(..).tuples() {
                if target_addr == 255 {
                    return y.to_string();
                } else if !(0..NUM_MACHINES as i128).contains(&target_addr) {
                    panic!("Attempted to send to unknown adddress {target_addr} ({x}/{y})")
                }
                let packets = packets_to_go.get_mut(target_addr as usize).unwrap();
                packets.push(x);
                packets.push(y);
            }
        }
    }
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "23",
    func: main,
    example_func: None,
});
