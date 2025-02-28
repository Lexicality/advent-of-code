// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;

use crate::aoc::y2019::computer::{Computer, RunState};

const NUM_MACHINES: usize = 50;

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let base_code: Computer = data.next().unwrap().parse().unwrap();

    let mut packets_to_go: Vec<Vec<i64>> = (0..NUM_MACHINES).map(|_| Vec::new()).collect();
    let mut computetrs: Vec<Computer> = (0..NUM_MACHINES)
        .map(|addr| {
            let mut computer = base_code.clone();
            computer.input.push_back(addr as i64);
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
                    return Ok(y.to_string());
                } else if !(0..NUM_MACHINES as i64).contains(&target_addr) {
                    panic!("Attempted to send to unknown adddress {target_addr} ({x}/{y})")
                }
                let packets = packets_to_go.get_mut(target_addr as usize).unwrap();
                packets.push(x);
                packets.push(y);
            }
        }
    }
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let base_code: Computer = data.next().unwrap().parse().unwrap();

    let mut packets_to_go: Vec<Vec<i64>> = (0..NUM_MACHINES).map(|_| Vec::new()).collect();
    let mut computetrs: Vec<Computer> = (0..NUM_MACHINES)
        .map(|addr| {
            let mut computer = base_code.clone();
            computer.input.push_back(addr as i64);
            computer
        })
        .collect();

    let mut last_nat_y = -1;
    let mut nat = (0, 0);
    let mut idle_timer = 0;
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
                    nat = (x, y);
                    continue;
                } else if !(0..NUM_MACHINES as i64).contains(&target_addr) {
                    panic!("Attempted to send to unknown adddress {target_addr} ({x}/{y})")
                }
                let packets = packets_to_go.get_mut(target_addr as usize).unwrap();
                packets.push(x);
                packets.push(y);
            }
        }

        if packets_to_go.iter().all(|packets| packets.is_empty()) {
            idle_timer += 1;
        } else {
            idle_timer = 0;
        }

        if idle_timer == 5 {
            idle_timer = 0;
            println!("Network idle! Sending x={},y={} to 0", nat.0, nat.1);
            if nat.1 == last_nat_y {
                return Ok(last_nat_y.to_string());
            }
            let packets = packets_to_go.get_mut(0).unwrap();
            packets.push(nat.0);
            packets.push(nat.1);
            last_nat_y = nat.1;
        }
    }
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "23",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
