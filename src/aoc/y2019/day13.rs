use std::fmt::Display;
use std::io::{stdin, stdout, Stdin, Write};

use itertools::Itertools;
use termion::cursor::HideCursor;
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

use crate::AoCError;

use super::computer::{Computer, Runstate};

enum GameScreen {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i128> for GameScreen {
    type Error = AoCError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
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

fn run(mut computer: Computer, stdinkeys: &mut Keys<Stdin>) -> i128 {
    let mut stdout = HideCursor::from(
        stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap(),
    );
    write!(stdout, "{}", termion::clear::All).unwrap();

    let mut backup_computer = computer.clone();
    let mut segment = 0;
    loop {
        let run_res = computer.run().unwrap();
        for (mut x, mut y, v) in computer.output.drain(..).tuples() {
            if x < 0 {
                segment = v;
                write!(stdout, "{}{}", termion::cursor::Goto(1, 1), segment).unwrap();
            } else {
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
        if matches!(run_res, Runstate::Finished) {
            break;
        }
        loop {
            let res = match stdinkeys.next().unwrap().unwrap() {
                Key::Left => -1,
                Key::Right => 1,
                Key::Char(' ') | Key::Char('\n') | Key::Up | Key::Down => 0,
                Key::Ctrl('c') | Key::Esc => return -1,
                Key::Char('s') => {
                    backup_computer = computer.clone();
                    continue;
                }
                Key::Char('r') => {
                    computer = backup_computer.clone();
                    break;
                }
                wat => {
                    write!(
                        stdout,
                        "{}{}? {wat:?}",
                        termion::cursor::Goto(0, 27),
                        termion::clear::UntilNewline
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                    continue;
                }
            };
            computer.input.push_back(res);
            break;
        }
    }

    segment
}

pub fn main(data: crate::DataIn) -> String {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    computer.set(0, 2.into());

    let stdin = stdin();
    let mut stdinkeys = stdin.keys();

    let mut segment;
    loop {
        segment = run(computer.clone(), &mut stdinkeys);
        if segment == -1 {
            return "^c".to_owned();
        }
        println!();
        println!("  ┏━━━━━━━━━━━┓");
        println!("  ┃ GAME OVER ┃");
        println!("  ┗━━━━━━━━━━━┛");
        println!();
        println!("Continue? [Y]/n");

        match stdinkeys.next().unwrap().unwrap() {
            Key::Char('Y') | Key::Char('y') | Key::Char(' ') | Key::Char('\n') => (),
            _ => break,
        }
    }

    segment.to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "13",
    func: main,
    example_func: None,
});
