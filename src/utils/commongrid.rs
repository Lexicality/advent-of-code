// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::fmt::Display;

use itertools::Itertools;
use num::{FromPrimitive, Signed, ToPrimitive};

use crate::{Coordinate, Coordinate2D};

use super::bigcoord2d::BigCoord2D;

pub trait CommonGrid<Key: Coordinate, Item> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn get(&self, k: &Key) -> Option<&Item>;
    fn get_mut(&mut self, k: &Key) -> Option<&mut Item>;
    fn set(&mut self, k: Key, v: Item) -> Option<Item>;
    fn get_or_set(&mut self, k: &Key, default: Item) -> &Item;

    fn min_key(&self) -> Key;
    fn max_key(&self) -> Key;
}

pub trait FlatGrid<Key: Coordinate2D, Item>: CommonGrid<Key, Item>
where
    Self: Sized + FromIterator<(Key, Item)>,
{
    fn new_from_lines<Iter, Inner>(data: Iter) -> Self
    where
        Inner: IntoIterator<Item = Item>,
        Iter: Iterator<Item = Inner>,
    {
        data.enumerate()
            .flat_map(|(y, inner)| {
                let y = Key::Type::from_usize(y).expect("Too many lines");
                inner.into_iter().enumerate().map(move |(x, item)| {
                    (
                        Key::from_tuple((Key::Type::from_usize(x).expect("Row is too big"), y)),
                        item,
                    )
                })
            })
            .collect()
    }

    fn new_from_iter<I>(data: I, width: Key::Type) -> Self
    where
        I: Iterator<Item = Item>,
    {
        let width = width.to_usize().expect("Width cannot be negative");
        Self::new_from_lines(data.chunks(width).into_iter())
    }
}

pub trait SparseGrid<Key: Coordinate2D, Item>: CommonGrid<Key, Item>
where
    Self: Sized + FromIterator<(Key, Item)>,
{
    fn new_from_sparse_lines<Iter, Inner>(data: Iter) -> Self
    where
        Inner: IntoIterator<Item = Option<Item>>,
        Iter: Iterator<Item = Inner>,
    {
        data.enumerate()
            .flat_map(|(y, inner)| {
                let y = Key::Type::from_usize(y).expect("Too many lines");
                inner.into_iter().enumerate().map(move |(x, item)| {
                    (
                        Key::from_tuple((Key::Type::from_usize(x).expect("Row is too big"), y)),
                        item,
                    )
                })
            })
            .filter_map(|(key, item)| item.map(|item| (key, item)))
            .collect()
    }

    fn new_from_sparse_iter<I>(data: I, width: Key::Type) -> Self
    where
        I: Iterator<Item = Option<Item>>,
    {
        let width = width.to_usize().expect("Width cannot be negative");
        Self::new_from_sparse_lines(data.chunks(width).into_iter())
    }
}

pub trait CharGrid<Key, Item>: FlatGrid<Key, Item>
where
    Key: Coordinate2D,
    Item: TryFrom<char>,
{
    fn new_from_chars<Input: IntoIterator<Item = String>>(
        data: Input,
    ) -> Result<Self, Item::Error> {
        let lines: Vec<Vec<Item>> = data
            .into_iter()
            .map(|line| line.chars().map(|c| c.try_into()).try_collect())
            .try_collect()?;
        Ok(Self::new_from_lines(lines.into_iter()))
    }
}

pub trait DisplayGrid<Key: Coordinate2D, Item>: CommonGrid<Key, Item> {
    fn get_for_display(&self, key: &Key) -> Option<&dyn Display>;

    fn do_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "[empty grid]");
        }
        let void = f.fill();

        let (min, max): (BigCoord2D, BigCoord2D) = {
            let min = self.min_key().to_tuple();
            let max = self.max_key().to_tuple();
            (
                (
                    min.0.to_i64().expect("Can't display coordinates >i64"),
                    min.1.to_i64().expect("Can't display coordinates >i64"),
                )
                    .into(),
                (
                    max.0.to_i64().expect("Can't display coordinates >i64"),
                    max.1.to_i64().expect("Can't display coordinates >i64"),
                )
                    .into(),
            )
        };

        // Iterator crimes thanks to Orman
        let mut forward_range;
        let mut back_range;
        let yrange: &mut dyn Iterator<Item = i64> = if f.sign_minus() {
            back_range = (min.y..=max.y).rev();
            &mut back_range
        } else {
            forward_range = min.y..=max.y;
            &mut forward_range
        };
        let mut left_pad = 0;
        if f.alternate() {
            let ymin = format!("{}", min.y).len();
            let ymax = format!("{}", max.y).len();
            left_pad = ymin.max(ymax);
            let xmin = format!("{}", min.x).len();
            let xmax = format!("{}", max.x).len();
            let top_pad = xmin.max(xmax);
            let char_iter = (min.x..=max.x).map(|x| {
                if x >= 0 {
                    format!("{x: >top_pad$}")
                } else {
                    format!("{: >top_pad$}", format!("╷{}", x.abs()))
                }
            });
            // Please don't judge me
            let x_axis: Vec<Vec<char>> = match top_pad {
                1 => {
                    let (a,) = char_iter
                        .map(|s| s.chars().collect_tuple().unwrap())
                        .multiunzip();
                    vec![a]
                }
                2 => {
                    let (a, b) = char_iter
                        .map(|s| s.chars().collect_tuple().unwrap())
                        .multiunzip();
                    vec![a, b]
                }
                3 => {
                    let (a, b, c) = char_iter
                        .map(|s| s.chars().collect_tuple().unwrap())
                        .multiunzip();
                    vec![a, b, c]
                }
                _ => panic!("Can't format x axis values with more than 3 characters"),
            };
            for line in x_axis {
                writeln!(
                    f,
                    "{: >left_pad$} {}",
                    ' ',
                    line.into_iter().collect::<String>()
                )?;
            }
            writeln!(
                f,
                "{: >left_pad$}┌{:─>xlen$}",
                ' ',
                '─',
                xlen = max.x.abs_sub(&min.x) as usize + 1
            )?;
        }

        for y in yrange {
            if f.alternate() {
                write!(f, "{y: >left_pad$}│")?;
            }
            for x in min.x..=max.x {
                self.get_for_display(&Key::from_tuple((
                    Key::Type::from_i64(x).expect("Key cannot go out of bounds"),
                    Key::Type::from_i64(y).expect("Key cannot go out of bounds"),
                )))
                .unwrap_or(&void)
                .fmt(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
