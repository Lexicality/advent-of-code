// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use itertools::Itertools;
use termion::cursor::HideCursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

use crate::{AoCError, InfGrid};

use super::computer::{Computer, RunState};

enum GameScreen {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i64> for GameScreen {
    type Error = AoCError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::Paddle),
            4 => Ok(Self::Ball),
            _ => Err(AoCError::new(format!("Unknown screen item {value}"))),
        }
    }
}

impl Display for GameScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.fill(),
            Self::Wall => '█',
            Self::Block => '▒',
            Self::Paddle => '═',
            Self::Ball => '❉',
        }
        .fmt(f)
    }
}

pub fn part_1(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    let res = computer.run().unwrap();
    assert!(matches!(res, RunState::Finished));

    let screen: InfGrid<GameScreen> = computer
        .output
        .drain(..)
        .tuples()
        .map(|(x, y, v)| {
            (
                (x.try_into().unwrap(), y.try_into().unwrap()).into(),
                v.try_into().unwrap(),
            )
        })
        .collect();

    println!("{screen}");

    Ok(screen
        .grid
        .values()
        .filter(|v| matches!(v, GameScreen::Block))
        .count()
        .to_string())
}

fn run(mut computer: Computer) -> i64 {
    let mut stdout = HideCursor::from(
        stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap(),
    );
    write!(stdout, "{}", termion::clear::All).unwrap();

    let stop = Arc::new(AtomicBool::new(false));
    let stop2 = stop.clone();
    std::thread::spawn(move || {
        for c in stdin().keys() {
            if let Key::Ctrl('c') = c.unwrap() {
                stop2.store(true, Ordering::Relaxed);
            }
        }
    });

    let mut segment = 0;
    let mut paddle_pos = 0;
    let mut ball_pos = 0;
    loop {
        let run_res = computer.run().unwrap();
        for (mut x, mut y, v) in computer.output.drain(..).tuples() {
            if x < 0 {
                segment = v;
                write!(stdout, "{}{}", termion::cursor::Goto(1, 1), segment).unwrap();
            } else {
                if v == 3 {
                    paddle_pos = x;
                } else if v == 4 {
                    ball_pos = x;
                }
                x += 1;
                y += 2;
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(x as u16, y as u16),
                    GameScreen::try_from(v).unwrap(),
                )
                .unwrap();
            }
        }
        stdout.flush().unwrap();
        if stop.load(Ordering::Relaxed) || matches!(run_res, RunState::Finished) {
            break;
        }
        let res = match paddle_pos.cmp(&ball_pos) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
            std::cmp::Ordering::Less => 1,
        };
        computer.input.push_back(res);
    }

    segment
}

pub fn part_2(mut data: crate::DataIn) -> crate::AoCResult<String> {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    computer.set(0, 2.into());

    Ok(run(computer).to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2019",
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
