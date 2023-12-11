use std::collections::HashMap;
use std::fmt::Display;

use crate::{Coord2D, Coordinate};

#[derive(Debug, Default)]
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
            return Ok(());
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
        for y in yrange {
            for x in self.min.x..=self.max.x {
                match self.get(&(x, y).into()) {
                    Some(item) => item.fmt(f)?,
                    None => write!(f, "{}", void)?,
                }
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
