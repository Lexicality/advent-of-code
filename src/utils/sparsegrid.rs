// Copyright (c) 2025 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;
use std::fmt::Display;

use crate::{
    CharGrid, CommonGrid, Coord2D, Coordinate, Coordinate2D, DisplayGrid, FlatGrid, GridState,
    VoidState,
};

#[derive(Debug, Default, Clone)]
pub struct SparseGrid<Item: VoidState = GridState, Key: Coordinate = Coord2D> {
    grid: HashMap<Key, Item>,
    default_instance: Item,
}

impl<Item: VoidState, Key: Coordinate> SparseGrid<Item, Key> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn entry(&mut self, key: Key) -> std::collections::hash_map::Entry<'_, Key, Item> {
        self.grid.entry(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Item)> {
        self.grid.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Key, &mut Item)> {
        self.grid.iter_mut()
    }

    pub fn get_or_set_default(&mut self, key: Key) -> &Item {
        self.entry(key).or_default()
    }
}

impl<Item: VoidState, Key: Coordinate> CommonGrid<Key, Item> for SparseGrid<Item, Key> {
    fn get(&self, key: &Key) -> Option<&Item> {
        Some(self.grid.get(key).unwrap_or(&self.default_instance))
    }

    fn get_mut(&mut self, key: &Key) -> Option<&mut Item> {
        self.grid.get_mut(key)
    }

    fn set(&mut self, key: Key, value: Item) -> Option<Item> {
        self.grid.insert(key, value)
    }

    fn get_or_set(&mut self, key: &Key, default: Item) -> &Item {
        self.entry(*key).or_insert(default)
    }

    fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }

    fn len(&self) -> usize {
        self.grid.len()
    }

    fn max_key(&self) -> Key {
        self.grid
            .keys()
            .copied()
            .reduce(|a, b| a.get_max(&b))
            .unwrap_or_default()
    }

    fn min_key(&self) -> Key {
        self.grid
            .keys()
            .copied()
            .reduce(|a, b| a.get_min(&b))
            .unwrap_or_default()
    }
}

impl<Item: VoidState, Key: Coordinate2D> FlatGrid<Key, Item> for SparseGrid<Item, Key> {}
impl<Item: VoidState + TryFrom<char>, Key: Coordinate2D> CharGrid<Key, Item>
    for SparseGrid<Item, Key>
{
}

impl<Item: VoidState + Display, Key: Coordinate2D> Display for SparseGrid<Item, Key> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.do_fmt(f)
    }
}

impl<Item: VoidState + Display, Key: Coordinate2D> DisplayGrid<Key, Item>
    for SparseGrid<Item, Key>
{
    fn get_for_display(&self, key: &Key) -> Option<&dyn Display> {
        self.get(key).map(|i| i as &dyn Display)
    }
}

impl<Item: VoidState, Key: Coordinate> FromIterator<(Key, Item)> for SparseGrid<Item, Key> {
    fn from_iter<T: IntoIterator<Item = (Key, Item)>>(iter: T) -> Self {
        Self {
            grid: iter
                .into_iter()
                .filter(|(_, item)| !item.is_void())
                .collect(),
            ..Default::default()
        }
    }
}

impl<Item: VoidState, Key: Coordinate> IntoIterator for SparseGrid<Item, Key> {
    type Item = (Key, Item);
    type IntoIter = std::collections::hash_map::IntoIter<Key, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter()
    }
}
