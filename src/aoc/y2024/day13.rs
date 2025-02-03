// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{AoCError, AoCResult};

type Stupid = i128;

const WAT: Stupid = 10000000000000;

#[derive(Debug)]
struct ClawMachine {
    a: [Stupid; 2],
    b: [Stupid; 2],
    prize: [Stupid; 2],
}

impl ClawMachine {
    fn new(data: &mut impl Iterator<Item = String>) -> AoCResult<Self> {
        let (a, b, prize) = data
            .next_tuple()
            .ok_or(AoCError::new("Not enough lines for a claw machine!"))?;
        lazy_static! {
            static ref BUTTON_RE: Regex = Regex::new(r"^Button .: X\+(\d+), Y\+(\d+)$").unwrap();
            static ref PRIZE_RE: Regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)").unwrap();
        }
        let matches_a = BUTTON_RE
            .captures(&a)
            .ok_or_else(|| AoCError::new(format!("Line '{a}' does not match regex!")))?;
        let matches_b = BUTTON_RE
            .captures(&b)
            .ok_or_else(|| AoCError::new(format!("Line '{b}' does not match regex!")))?;
        let matches_prize = PRIZE_RE
            .captures(&prize)
            .ok_or_else(|| AoCError::new(format!("Line '{prize}' does not match regex!")))?;

        Ok(Self {
            a: [
                matches_a[1]
                    .parse()
                    ?,
                matches_a[2]
                    .parse()
                    ?,
            ],
            b: [
                matches_b[1]
                    .parse()
                    ?,
                matches_b[2]
                    .parse()
                    ?,
            ],
            prize: [
                matches_prize[1]
                    .parse()
                    ?,
                matches_prize[2]
                    .parse()
                    ?,
            ],
        })
    }

    fn factorio(&self, addition: Stupid) -> Option<(Stupid, Stupid)> {
        let a_1 = self.a[0] * self.b[1];
        let p_1 = (addition + self.prize[0]) * self.b[1];
        let a_2 = self.a[1] * self.b[0];
        let p_2 = (addition + self.prize[1]) * self.b[0];
        let a = a_1 - a_2;
        let p = p_1 - p_2;
        if p % a != 0 {
            println!("Uneven division p % a = {}", p % a);
            return None;
        }
        let a = p / a;
        if a < 0 {
            println!("A is negative! {a}");
            return None;
        }
        let b = (addition + self.prize[0]) - a * self.a[0];
        let b_c = self.b[0];
        if b % b_c != 0 {
            println!("Uneven division b % b_c = {}", b % b_c);
            return None;
        }
        let b = b / b_c;
        if b < 0 {
            println!("B is negative! {b}");
            return None;
        }
        Some((a, b))
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut data = data.peekable();

    let mut ret = 0;
    while data.peek().is_some() {
        let claw = ClawMachine::new(&mut data)?;
        println!("{claw:?}");
        if let Some((a, b)) = claw.factorio(0) {
            println!("a: {a} b: {b}");

            if a <= 100 && b <= 100 {
                ret += b + a * 3;
            }
        }
        if !data.next().is_none_or(|line| line.is_empty()) {
            return Err(AoCError::new("Got out of sync with the claws!"));
        }
    }

    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut data = data.peekable();

    let mut ret = 0;
    while data.peek().is_some() {
        let claw = ClawMachine::new(&mut data)?;
        println!("{claw:?}");
        if let Some((a, b)) = claw.factorio(WAT) {
            println!("a: {a} b: {b}");
            ret += b + a * 3;
        }
        if !data.next().is_none_or(|line| line.is_empty()) {
            return Err(AoCError::new("Got out of sync with the claws!"));
        }
    }

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "13",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
