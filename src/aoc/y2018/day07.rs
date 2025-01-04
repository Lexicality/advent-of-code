// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, AoCResult};

type StepID = char;

#[derive(Debug, Default, Clone)]
struct Step {
    dependencies: Vec<StepID>,
    dependants: Vec<StepID>,
}

fn parse_steps(data: crate::DataIn) -> AoCResult<HashMap<StepID, Step>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^Step (.) must be finished before step (.) can begin.$").unwrap();
    }

    let mut steps: HashMap<StepID, Step> = HashMap::new();
    for line in data {
        let matches = RE
            .captures(&line)
            .ok_or_else(|| AoCError::new(format!("input {line} does not match regex")))?;

        let before = matches[1].chars().next().unwrap();
        let after = matches[2].chars().next().unwrap();

        steps.entry(before).or_default().dependants.push(after);
        steps.entry(after).or_default().dependencies.push(before);
    }
    Ok(steps)
}

fn alphasort((id, _): &(&char, &Step)) -> impl Ord {
    u32::from('Z') - u32::from(**id)
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let steps = parse_steps(data)?;
    let mut done = HashSet::with_capacity(steps.len());

    // let's see if I can be smart here
    let mut to_do = Vec::with_capacity(steps.len());
    to_do.extend(
        steps
            .iter()
            .filter(|(_, step)| step.dependencies.is_empty()),
    );
    to_do.sort_by_cached_key(alphasort);

    let mut ret = String::with_capacity(steps.len());

    while let Some((step_id, step)) = to_do.pop() {
        done.insert(step_id);
        ret.push(*step_id);
        for step_id in step.dependants.iter() {
            let step = steps.get(step_id).unwrap();
            if step
                .dependencies
                .iter()
                .all(|step_id| done.contains(step_id))
            {
                to_do.push((step_id, step));
            }
        }
        to_do.sort_by_cached_key(alphasort);
    }

    Ok(ret.to_string())
}

fn get_time(id: StepID) -> u32 {
    let aa = u32::from('A') - 1;
    u32::from(id) - aa
}

struct WorkerPool {
    workers: Vec<Option<(StepID, u32)>>,
}

impl WorkerPool {
    fn new(num: usize) -> Self {
        Self {
            workers: (0..num).map(|_| None).collect(),
        }
    }

    fn tick(&mut self) -> impl Iterator<Item = StepID> + use<'_> {
        self.workers
            .iter_mut()
            .flat_map(|worker| {
                worker.take_if(|(_, time)| {
                    *time -= 1;
                    *time == 0
                })
            })
            .map(|(id, _)| id)
    }

    fn has_capacity(&self) -> bool {
        self.workers.iter().any(|worker| worker.is_none())
    }

    fn start_working(&mut self, step_id: StepID, time: u32) {
        for worker in self.workers.iter_mut() {
            if worker.is_none() {
                let _ = worker.insert((step_id, time));
                return;
            }
        }
        unreachable!("no free workers?!")
    }
}

pub fn part_2(data: crate::DataIn, num_helpers: usize, time_penalty: u32) -> AoCResult<String> {
    let steps = parse_steps(data)?;
    let num_steps = steps.len();
    let mut done = HashSet::with_capacity(num_steps);

    let mut to_do = Vec::with_capacity(steps.len());
    to_do.extend(
        steps
            .iter()
            .filter(|(_, step)| step.dependencies.is_empty()),
    );
    to_do.sort_by_cached_key(alphasort);

    let mut helpers = WorkerPool::new(num_helpers);
    let mut time = 0;

    while done.len() < num_steps {
        let steps_done = helpers.tick();
        for step_id in steps_done {
            let step = steps.get(&step_id).unwrap();
            done.insert(step_id);
            for step_id in step.dependants.iter() {
                let step = steps.get(step_id).unwrap();
                if step
                    .dependencies
                    .iter()
                    .all(|step_id| done.contains(step_id))
                {
                    to_do.push((step_id, step));
                }
            }
        }

        while !to_do.is_empty() && helpers.has_capacity() {
            to_do.sort_by_cached_key(alphasort);
            let (step_id, _) = to_do.pop().unwrap();
            helpers.start_working(*step_id, get_time(*step_id) + time_penalty);
        }
        time += 1;
    }

    // hmm
    time -= 1;

    Ok(time.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2018",
    day: "7",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: |data| part_2(data, 5, 60),
        example: |data| part_2(data, 2, 0),
    })
});
