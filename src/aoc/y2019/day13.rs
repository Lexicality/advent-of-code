use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCError, InfGrid};

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
            Self::Paddle => '▃',
            Self::Ball => '▚',
        }
        .fmt(f)
    }
}

pub fn main(data: crate::DataIn) -> String {
    let mut computer: Computer = data.next().unwrap().parse().unwrap();
    let res = computer.run().unwrap();
    assert!(matches!(res, Runstate::Finished));

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

    screen
        .grid
        .values()
        .filter(|v| matches!(v, GameScreen::Block))
        .count()
        .to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2019",
    day: "13",
    func: main,
    example_func: None,
});
