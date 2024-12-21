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
        m.insert(
            (KB::Left, KB::A),
            vec![KB::Right, KB::Right, KB::Right, KB::Up, KB::A],
        );
        m.insert((KB::Left, KB::Right), vec![KB::Right, KB::Right, KB::A]);
        m.insert((KB::Left, KB::Down), vec![KB::Right, KB::A]);
        m.insert((KB::Left, KB::Up), vec![KB::Right, KB::Up, KB::A]);
        m
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
        KEYPAD_MOVEMENTS
            .get(&(id.last_key, id.key_to_get_here))
            .expect("all of these are defined")
            .len()
            .try_into()
            .unwrap()
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

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
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
            // println!(
            //     "Going from {a:?}({}) to {b:?}({})",
            //     provider.start, provider.end
            // );
            let start = provider.get_start();
            let route = a_star(provider, start);
            assert!(!route.is_empty());
            movements.extend(route.into_iter().rev().map(|v| v.key_to_get_here));
            movements.push(KB::A);

            // route.reverse();
            // for v in route {
            //     // println!(
            //     //     "Pos {} going from {} to {}",
            //     //     v.pos, v.last_key, v.key_to_get_here
            //     // );
            //     print!("{}", v.key_to_get_here);
            // }
            // // println!("???");
            // print!("A");
        }
        movements = expandinate(movements);
        movements = expandinate(movements);
        let num_moves = movements.len();
        println!(
            "{line}: {} (should be {}) {}",
            num_moves,
            {
                match line.as_str() {
                    "029A" => {
                        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
                    }
                    "980A" => "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len(),
                    "179A" => {
                        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
                    }
                    "456A" => {
                        "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
                    }
                    "379A" => {
                        "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
                    }
                    _ => 0,
                }
            },
            movements.into_iter().join("")
        );
    }
    let ret = "";
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2024", "21", main));
