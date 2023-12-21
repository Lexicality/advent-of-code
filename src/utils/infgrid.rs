use std::collections::HashMap;
use std::fmt::Display;

use crate::{CharGrid, CommonGrid, Coord2D, Coordinate, Coordinate2D, DisplayGrid};

use super::commongrid::FlatGrid;

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
        self.do_fmt(f)
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

impl<Item, Key: Coordinate> CommonGrid<Key, Item> for InfGrid<Item, Key> {
    fn get(&self, k: &Key) -> Option<&Item> {
        self.grid.get(k)
    }

    fn get_mut(&mut self, k: &Key) -> Option<&mut Item> {
        self.grid.get_mut(k)
    }

    fn set(&mut self, k: Key, v: Item) -> Option<Item> {
        self.min = self.min.get_min(&k);
        self.max = self.max.get_max(&k);
        self.grid.insert(k, v)
    }

    fn get_or_set(&mut self, k: &Key, default: Item) -> &Item {
        if !self.grid.contains_key(k) {
            self.set(k.to_owned(), default);
        }
        self.get(k).unwrap()
    }

    fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }

    fn len(&self) -> usize {
        self.grid.len()
    }

    fn max_key(&self) -> Key {
        self.max
    }

    fn min_key(&self) -> Key {
        self.min
    }
}

impl<Item, Key: Coordinate2D> FlatGrid<Key, Item> for InfGrid<Item, Key> {}
impl<Item: TryFrom<char>, Key: Coordinate2D> CharGrid<Key, Item> for InfGrid<Item, Key> {}

// impl<Item, Key: Coordinate2D> DisplayGrid<Key, Item> for InfGrid<Item, Key> {
//     fn get_for_display(&self, key: &Key) -> Option<&dyn Display> {
//         self.get(key).map(&'#')
//     }
// }

impl<Item: Display, Key: Coordinate2D> DisplayGrid<Key, Item> for InfGrid<Item, Key> {
    fn get_for_display(&self, key: &Key) -> Option<&dyn Display> {
        self.get(key).map(|i| i as &dyn Display)
    }
}
