use itertools::Itertools;

use super::computer::Computer;

pub fn main_example(data: crate::DataIn) -> String {
    let mut ret = 0;
    while let Some(phase) = data.next() {
        let og_computer: Computer = data.next().unwrap().parse().unwrap();

        let phase_setting = phase.split(',').map(|v| v.parse().unwrap()).collect_vec();
        let mut signal = 0;
        for phase in phase_setting.iter() {
            let mut amplifier = og_computer.clone();
            amplifier.input.push_front(signal);
            amplifier.input.push_front(*phase);
            amplifier.run().unwrap();
            let output = amplifier.output[0];
            println!("signal: {signal}, phase: {phase}, output: {output}");
            signal = output;
        }
        ret = ret.max(signal);
    }
    ret.to_string()
}

pub fn main(data: crate::DataIn) -> String {
    let mut ret = 0;
    let og_computer: Computer = data.next().unwrap().parse().unwrap();

    for phase_setting in (0..5).permutations(5) {
        let mut signal = 0;
        for phase in phase_setting.iter() {
            let mut amplifier = og_computer.clone();
            amplifier.input.push_front(signal);
            amplifier.input.push_front(*phase);
            amplifier.run().unwrap();
            let output = amplifier.output[0];
            signal = output;
        }
        ret = ret.max(signal);
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "7",
    func: main,
});
