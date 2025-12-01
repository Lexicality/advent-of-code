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

use crate::AoCError;

#[derive(Debug)]
enum Instruction {
    Left(i16),
    Right(i16),
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
        write!(
            f,
            "Rotate {} {} clicks",
            match self {
                Self::Left(_) => "left",
                Self::Right(_) => "right",
            },
            match self {
                Self::Left(i) => i,
                Self::Right(i) => i,
            }
        )
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
    fn rotate(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Left(count) => {
                let count: i64 = count.into();
                self.0 -= count;
            }
            Instruction::Right(count) => {
                let count: i64 = count.into();
                self.0 += count;
            }
        }
        self.clampinate();
    }

    fn clampinate(&mut self) {
        while self.0 < 0 {
            self.0 += 100;
        }
        while self.0 > 99 {
            self.0 -= 100;
        }
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    let mut dial: Dial = Default::default();
    for line in data {
        let instruction = line.parse()?;
        log::debug!("{dial}");
        log::debug!("-> {instruction}");
        dial.rotate(instruction);
        if dial.0 == 0 {
            ret += 1;
        }
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
    part_2: None
});
