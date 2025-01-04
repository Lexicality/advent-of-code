// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::utils::astar::{a_star, AStarProvider};
use crate::{AoCError, CommonGrid, Coord2D, Coordinate, InfGrid};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum KB {
    A,
    Up,
    Down,
    Left,
    Right,
}

impl From<KB> for Coord2D {
    fn from(value: KB) -> Self {
        match value {
            KB::Up => Coord2D { x: 0, y: -1 },
            KB::Right => Coord2D { x: 1, y: 0 },
            KB::Down => Coord2D { x: 0, y: 1 },
            KB::Left => Coord2D { x: -1, y: 0 },
            KB::A => Coord2D { x: 0, y: 0 },
        }
    }
}

impl TryFrom<Coord2D> for KB {
    type Error = AoCError;
    fn try_from(value: Coord2D) -> Result<Self, Self::Error> {
        match value {
            Coord2D { x: 0, y: 0 } => Ok(Self::A),
            Coord2D { x: 0, y } if y < 0 => Ok(Self::Up),
            Coord2D { x, y: 0 } if x > 0 => Ok(Self::Right),
            Coord2D { x: 0, y } if y > 0 => Ok(Self::Down),
            Coord2D { x, y: 0 } if x < 0 => Ok(Self::Left),
            _ => Err(AoCError::new("Diagonals are unsupported")),
        }
    }
}

impl Display for KB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KB::A => 'A',
            KB::Up => '^',
            KB::Down => 'v',
            KB::Left => '<',
            KB::Right => '>',
        }
        .fmt(f)
    }
}

lazy_static! {
    static ref KEYPAD_MOVEMENTS: HashMap<(KB, KB), Vec<KB>> = {
        let mut m = HashMap::new();
        for btn in [
            KB::A,
            KB::Up,
            KB::Down,
            KB::Left,
            KB::Right,
        ] {
            m.insert((btn, btn), vec![KB::A]);
        }
        // I did this by hand because I'm an idiot
        m.insert((KB::A, KB::Up), vec![KB::Left, KB::A]);
        m.insert((KB::A, KB::Right), vec![KB::Down, KB::A]);
        m.insert((KB::A, KB::Down), vec![KB::Down, KB::Left, KB::A]);
        m.insert((KB::A, KB::Left), vec![KB::Down, KB::Left, KB::Left, KB::A]);
        m.insert((KB::Up, KB::A), vec![KB::Right, KB::A]);
        m.insert((KB::Up, KB::Down), vec![KB::Down, KB::A]);
        m.insert((KB::Up, KB::Right), vec![KB::Right, KB::Down, KB::A]);
        m.insert((KB::Up, KB::Left), vec![KB::Down, KB::Left, KB::A]);
        m.insert((KB::Right, KB::A), vec![KB::Up, KB::A]);
        m.insert((KB::Right, KB::Down), vec![KB::Left, KB::A]);
        m.insert((KB::Right, KB::Left), vec![KB::Left, KB::Left, KB::A]);
        m.insert((KB::Right, KB::Up), vec![KB::Up, KB::Left, KB::A]);
        m.insert((KB::Down, KB::A), vec![KB::Up, KB::Right, KB::A]);
        m.insert((KB::Down, KB::Right), vec![KB::Right, KB::A]);
        m.insert((KB::Down, KB::Left), vec![KB::Left, KB::A]);
        m.insert((KB::Down, KB::Up), vec![KB::Up, KB::A]);
        m.insert((KB::Left, KB::A), vec![KB::Right, KB::Right, KB::Up, KB::A]);
        m.insert((KB::Left, KB::Right), vec![KB::Right, KB::Right, KB::A]);
        m.insert((KB::Left, KB::Down), vec![KB::Right, KB::A]);
        m.insert((KB::Left, KB::Up), vec![KB::Right, KB::Up, KB::A]);
        m
    };
    static ref EXPANDED_KEYPAD_MOVEMENTS: HashMap<(KB, KB), u64> = {
        KEYPAD_MOVEMENTS
            .iter()
            .map(|(key, moves)| {
                (
                    *key,
                    expandinate(expandinate(
                        moves
                            .iter()
                            .copied()
                            .chain(
                                KEYPAD_MOVEMENTS
                                    .get(&(key.1, KB::A))
                                    .unwrap()
                                    .iter()
                                    .copied(),
                            )
                            .collect(),
                    ))
                    .len()
                    .try_into()
                    .unwrap(),
                )
            })
            .collect()
    };
}

#[derive(Debug, Clone, Copy)]
enum NumericKeypad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
}

impl TryFrom<char> for NumericKeypad {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'A' => Self::A,
            _ => return Err(AoCError::new_from_char(value)),
        })
    }
}

impl From<NumericKeypad> for Coord2D {
    fn from(value: NumericKeypad) -> Self {
        match value {
            NumericKeypad::One => Coord2D { x: 0, y: 2 },
            NumericKeypad::Two => Coord2D { x: 1, y: 2 },
            NumericKeypad::Three => Coord2D { x: 2, y: 2 },
            NumericKeypad::Four => Coord2D { x: 0, y: 1 },
            NumericKeypad::Five => Coord2D { x: 1, y: 1 },
            NumericKeypad::Six => Coord2D { x: 2, y: 1 },
            NumericKeypad::Seven => Coord2D { x: 0, y: 0 },
            NumericKeypad::Eight => Coord2D { x: 1, y: 0 },
            NumericKeypad::Nine => Coord2D { x: 2, y: 0 },
            NumericKeypad::Zero => Coord2D { x: 1, y: 3 },
            NumericKeypad::A => Coord2D { x: 2, y: 3 },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct AStarID {
    pos: Coord2D,
    last_key: KB,
    key_to_get_here: KB,
}

#[derive(Debug, Clone)]
struct AStarImpl<'a> {
    grid: &'a InfGrid<NumericKeypad>,
    start: Coord2D,
    end: Coord2D,
}

impl AStarImpl<'_> {
    fn get_start(&self) -> AStarID {
        AStarID {
            pos: self.start,
            last_key: KB::A,
            key_to_get_here: KB::A,
        }
    }
}

impl AStarProvider for AStarImpl<'_> {
    type IDType = AStarID;

    fn get_neighbours(&self, id: &Self::IDType) -> Box<dyn Iterator<Item = Self::IDType> + '_> {
        let AStarID {
            pos,
            last_key: _,
            key_to_get_here: last_key,
        } = *id;
        Box::new(
            [KB::Up, KB::Down, KB::Left, KB::Right]
                .into_iter()
                .map(move |key| AStarID {
                    pos: pos + key.into(),
                    last_key,
                    key_to_get_here: key,
                })
                .filter(|id| self.grid.get(&id.pos).is_some()),
        )
    }

    fn heuristic(&self, id: &Self::IDType) -> u64 {
        self.end.distance(&id.pos).try_into().unwrap()
    }

    fn cost(&self, id: &Self::IDType) -> u64 {
        EXPANDED_KEYPAD_MOVEMENTS
            .get(&(id.last_key, id.key_to_get_here))
            .copied()
            .expect("all of these are defined")
    }

    fn is_end(&self, id: &Self::IDType) -> bool {
        id.pos == self.end
    }
}

fn expandinate(movements: Vec<KB>) -> Vec<KB> {
    let starter = [KB::A];
    starter
        .into_iter()
        .chain(movements)
        .tuple_windows()
        .flat_map(|(a, b)| {
            KEYPAD_MOVEMENTS
                .get(&(a, b))
                .expect("all of these are defined")
                .to_owned()
        })
        .collect()
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let keygrid: InfGrid<_> = [
        NumericKeypad::One,
        NumericKeypad::Two,
        NumericKeypad::Three,
        NumericKeypad::Four,
        NumericKeypad::Five,
        NumericKeypad::Six,
        NumericKeypad::Seven,
        NumericKeypad::Eight,
        NumericKeypad::Nine,
        NumericKeypad::Zero,
        NumericKeypad::A,
    ]
    .into_iter()
    .map(|k| (k.into(), k))
    .collect();

    let starter = [NumericKeypad::A];

    let mut ret = 0;
    for line in data {
        let mut movements = Vec::new();
        for (a, b) in starter
            .iter()
            .copied()
            .chain(line.chars().map(|c| c.try_into().unwrap()))
            .tuple_windows()
        {
            let provider = AStarImpl {
                grid: &keygrid,
                start: a.into(),
                end: b.into(),
            };
            let start = provider.get_start();
            let route = a_star(provider, start);
            assert!(!route.is_empty());
            movements.extend(route.into_iter().rev().map(|v| v.key_to_get_here));
            movements.push(KB::A);
        }
        // println!("{line}: {}", movements.iter().join(""));
        movements = expandinate(movements);
        // println!("{line}: {}", movements.iter().join(""));
        movements = expandinate(movements);
        // println!("{line}: {}", movements.iter().join(""));
        let num_moves = movements.len();
        let keypad_num: usize = line[..3].parse().unwrap();
        println!("{line}: {}*{}", num_moves, keypad_num,);
        ret += num_moves * keypad_num;
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "21",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None,
});
