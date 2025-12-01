// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;
use std::str::FromStr;

use num::Integer;

use crate::AoCError;

#[derive(Debug)]
enum Instruction {
    Left(u16),
    Right(u16),
}

impl FromStr for Instruction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count) = s
            .split_at_checked(1)
            .ok_or(AoCError::new("Unexpected utf sequence"))?;

        let count = count.parse().map_err(AoCError::new_from_parseerror)?;

        Ok(match dir {
            "L" => Self::Left(count),
            "R" => Self::Right(count),
            c => return Err(AoCError::new(format!("Unexpected direction {c}!"))),
        })
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (dir, amt) = match self {
            Self::Left(i) => ("L", i),
            Self::Right(i) => ("R", i),
        };
        write!(f, "{}{}", dir, amt)
    }
}

#[derive(Debug)]
struct Dial(i64);

impl Default for Dial {
    fn default() -> Self {
        Self(50)
    }
}

impl Display for Dial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dial is at {}", self.0)
    }
}

impl Dial {
    fn rotate(&mut self, instruction: Instruction) -> u32 {
        let mut ret = 0;

        match instruction {
            Instruction::Left(count) => {
                let (full_rotations, count) = count.div_mod_floor(&100);
                ret += full_rotations as u32;
                let count: i64 = count.into();
                let prezero = self.0 == 0;
                self.0 -= count;
                if self.0 < 0 {
                    self.0 += 100;
                    if !prezero {
                        ret += 1;
                    }
                } else if count > 0 && self.0 == 0 {
                    ret += 1;
                }
            }
            Instruction::Right(count) => {
                let (full_rotations, count) = count.div_mod_floor(&100);
                ret += full_rotations as u32;
                let count: i64 = count.into();
                self.0 += count;
                if self.0 > 99 {
                    self.0 -= 100;
                    ret += 1;
                } else if count > 0 && self.0 == 0 {
                    ret += 1;
                }
            }
        }
        if ret > 0 {
            log::debug!(
                "The dial is rotated {instruction} to point at {}; {ret} passes of 0",
                self.0
            );
        } else {
            log::debug!("The dial is rotated {instruction} to point at {}", self.0);
        }

        ret
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    let mut dial: Dial = Default::default();
    for line in data {
        let instruction = line.parse()?;
        dial.rotate(instruction);
        if dial.0 == 0 {
            ret += 1;
        }
    }
    log::info!("{dial}");
    Ok(ret.to_string())
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    let mut dial: Dial = Default::default();
    for line in data {
        let instruction = line.parse()?;
        ret += dial.rotate(instruction);
    }
    log::info!("{dial}");
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2025",
    day: "1",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    })
});

#[cfg(test)]
mod test {
    use super::{Dial, Instruction};

    #[test]
    fn test_safe_zero() {
        let mut dial = Dial(50);
        let instruction = Instruction::Left(50);

        let ret = dial.rotate(instruction);
        assert_eq!(dial.0, 0);
        assert_eq!(ret, 1);
    }

    #[test]
    fn test_right_zero() {
        let mut dial = Dial(50);
        let instruction = Instruction::Right(50);

        let ret = dial.rotate(instruction);
        assert_eq!(dial.0, 0);
        assert_eq!(ret, 1);
    }

    #[test]
    fn test_left_multi_rot() {
        let mut dial = Dial(50);
        let instruction = Instruction::Left(150);

        let ret = dial.rotate(instruction);
        assert_eq!(dial.0, 0);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_right_multi_rot() {
        let mut dial = Dial(50);
        let instruction = Instruction::Right(150);

        let ret = dial.rotate(instruction);
        assert_eq!(dial.0, 0);
        assert_eq!(ret, 2);
    }

    #[test]
    fn test_zero_full_left() {
        let mut dial = Dial(0);
        let instruction = Instruction::Left(100);

        let ret = dial.rotate(instruction);
        assert_eq!(dial.0, 0);
        assert_eq!(ret, 1);
    }

    #[test]
    fn test_zero_full_right() {
        let mut dial = Dial(0);
        let instruction = Instruction::Right(100);

        let ret = dial.rotate(instruction);
        assert_eq!(dial.0, 0);
        assert_eq!(ret, 1);
    }

    #[test]
    fn test_one_click_left() {
        let mut dial = Dial(0);
        let instruction = Instruction::Left(1);

        let ret = dial.rotate(instruction);
        assert_eq!(dial.0, 99);
        assert_eq!(ret, 0);
    }

    #[test]
    fn test_one_click_right() {
        let mut dial = Dial(0);
        let instruction = Instruction::Right(1);

        let ret = dial.rotate(instruction);
        assert_eq!(dial.0, 1);
        assert_eq!(ret, 0);
    }
}
