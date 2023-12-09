use itertools::Itertools;

use crate::AoCResult;

use super::computer::{Computer, Runstate};

fn run_amps(program: Computer, phase_settings: &[i128]) -> AoCResult<i128> {
    let mut loop_signals = vec![0];
    let mut amps = phase_settings
        .iter()
        .map(|phase| {
            let mut amp = program.clone();
            amp.input.push_front(*phase);
            amp
        })
        .collect::<Vec<_>>();
    loop {
        let mut halts = 0;
        for amp in amps.iter_mut() {
            // println!("{amp}");
            assert!(!loop_signals.is_empty(), "no loop data!");
            // println!("input: {loop_signals:?}");
            amp.input.extend(loop_signals.drain(..));
            let ret = amp.run()?;
            loop_signals.append(&mut amp.output);
            // println!("output: {loop_signals:?}");
            if matches!(ret, Runstate::Finished) {
                halts += 1;
            }
        }
        if halts == phase_settings.len() {
            assert_eq!(loop_signals.len(), 1, "too many signals left!");
            return Ok(loop_signals[0]);
        } else if halts != 0 {
            panic!("partial halt!!");
        }
    }
}

pub fn main_example(data: crate::DataIn) -> String {
    let mut ret = 0;
    while let Some(phase) = data.next() {
        let og_computer: Computer = data.next().unwrap().parse().unwrap();

        let phase_settings = phase.split(',').map(|v| v.parse().unwrap()).collect_vec();
        let signal = run_amps(og_computer, &phase_settings).unwrap();
        println!("{phase_settings:?} {signal}");
        ret = ret.max(signal);
    }
    ret.to_string()
}

pub fn main(data: crate::DataIn) -> String {
    let mut ret = 0;
    let og_computer: Computer = data.next().unwrap().parse().unwrap();

    for phase_setting in (5..10).permutations(5) {
        let signal = run_amps(og_computer.clone(), &phase_setting).unwrap();
        ret = ret.max(signal);
    }
    ret.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "7",
    func: main,
    example_func: Some(main_example),
});
