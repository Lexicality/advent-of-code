use std::collections::HashMap;
use std::fmt::Display;

use itertools::Itertools;
use num::Signed;

use crate::{Coord2D, Coordinate};

#[derive(Debug, Default, Clone)]
pub struct InfGrid<Item, Key: Coordinate = Coord2D> {
    pub grid: HashMap<Key, Item>,
    pub min: Key,
    pub max: Key,
}

impl<Key: Coordinate, Item> InfGrid<Item, Key> {
    pub fn new() -> Self {
        InfGrid {
            grid: HashMap::new(),
            min: Key::MAX,
            max: Key::MIN,
        }
    }

    pub fn get(&self, k: &Key) -> Option<&Item> {
        self.grid.get(k)
    }

    pub fn set(&mut self, k: Key, v: Item) {
        self.min = self.min.get_min(&k);
        self.max = self.max.get_max(&k);
        self.grid.insert(k, v);
    }

    pub fn get_or_set(&mut self, k: &Key, default: Item) -> &Item {
        if !self.grid.contains_key(k) {
            self.set(k.to_owned(), default);
        }
        self.get(k).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Item)> {
        self.grid.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Key, &mut Item)> {
        self.grid.iter_mut()
    }
}

impl<Key: Coordinate, Item: Default> InfGrid<Item, Key> {
    pub fn get_or_set_default(&mut self, k: &Key) -> &Item {
        self.get_or_set(k, Default::default())
    }
}

impl<Item: Display> Display for InfGrid<Item, Coord2D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.grid.is_empty() {
            return write!(f, "[empty grid]");
        }
        let void = f.fill();
        // Iterator crimes thanks to Orman
        let mut forward_range;
        let mut back_range;
        let yrange: &mut dyn Iterator<Item = i32> = if f.sign_minus() {
            back_range = (self.min.y..=self.max.y).rev();
            &mut back_range
        } else {
            forward_range = self.min.y..=self.max.y;
            &mut forward_range
        };
        let mut left_pad = 0;
        if f.alternate() {
            let ymin = format!("{}", self.min.y).len();
            let ymax = format!("{}", self.max.y).len();
            left_pad = ymin.max(ymax);
            let xmin = format!("{}", self.min.x).len();
            let xmax = format!("{}", self.max.x).len();
            let top_pad = xmin.max(xmax);
            let char_iter = (self.min.x..=self.max.x).map(|x| {
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
                xlen = self.max.x.abs_sub(&self.min.x) as usize + 1
            )?;
        }

        for y in yrange {
            if f.alternate() {
                write!(f, "{y: >left_pad$}│")?;
            }
            for x in self.min.x..=self.max.x {
                self.get(&(x, y).into())
                    .map(|i| i as &dyn Display)
                    .unwrap_or(&void)
                    .fmt(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<Item, Key: Coordinate> FromIterator<(Key, Item)> for InfGrid<Item, Key> {
    fn from_iter<T: IntoIterator<Item = (Key, Item)>>(iter: T) -> Self {
        let mut min = Key::MAX;
        let mut max = Key::MIN;
        let grid = iter
            .into_iter()
            .inspect(|(coord, _)| {
                min = min.get_min(coord);
                max = max.get_max(coord);
            })
            .collect();
        Self { grid, min, max }
    }
}

impl<Item, Key: Coordinate> IntoIterator for InfGrid<Item, Key> {
    type Item = (Key, Item);
    type IntoIter = std::collections::hash_map::IntoIter<Key, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter()
    }
}

impl<Item> InfGrid<Item, Coord2D> {
    pub fn new_from_lines<Iter, Inner>(data: Iter) -> Self
    where
        Inner: IntoIterator<Item = Item>,
        Iter: Iterator<Item = Inner>,
    {
        data.enumerate()
            .flat_map(|(y, inner)| {
                inner
                    .into_iter()
                    .enumerate()
                    .map(move |(x, item)| ((x, y).try_into().unwrap(), item))
            })
            .collect()
    }
}
