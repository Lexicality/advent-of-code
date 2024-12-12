// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

use crate::{CharGrid, CommonGrid, Coord2D, Coordinate, DisplayGrid, FlatGrid};

#[derive(Debug, Clone)]
pub struct Grid<Item> {
    pub grid: HashMap<Coord2D, Item>,
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
impl<Item> Grid<Item> {
    pub fn new_with_initialiser<F: Fn() -> Item>(width: u32, height: u32, init: F) -> Self {
        Self::validate_dimensions(width, height);
        Self {
            grid: (0..width)
                .cartesian_product(0..height)
                .map(|(x, y)| ((x, y).try_into().unwrap(), init()))
                .collect(),
            width,
            height,
        }
    }

    fn validate_dimensions(width: u32, height: u32) -> usize {
        assert!(width <= (i32::MAX as u32), "grid is too wide!");
        assert!(height <= (i32::MAX as u32), "grid is too tall!");
        (width as usize)
            .checked_mul(height as usize)
            .expect("Grid is too big!")
    }

    pub fn new_empty(width: u32, height: u32) -> Self {
        let size = Self::validate_dimensions(width, height);
        Self {
            grid: HashMap::with_capacity(size),
            width,
            height,
        }
    }

    pub fn check_coord(&self, coord: &Coord2D) -> bool {
        (coord.x >= 0 && coord.y >= 0)
            && ((coord.x as u32) < self.width && (coord.y as u32) < self.height)
    }

    pub fn get_row(&self, row: u32) -> Vec<(Coord2D, &Item)> {
        assert!(row < self.height);
        (0..self.width)
            .map(move |x| {
                let coord = (x, row).try_into().unwrap();
                (coord, self.get(&coord).unwrap())
            })
            .collect()
    }

    pub fn get_column(&self, column: u32) -> Vec<(Coord2D, &Item)> {
        assert!(column < self.width);
        (0..self.height)
            .map(move |y| {
                let coord = (column, y).try_into().unwrap();
                (coord, self.get(&coord).unwrap())
            })
            .collect()
    }

    pub fn get_neighbour_coords(
        &self,
        coord: Coord2D,
        diagonal: bool,
    ) -> impl Iterator<Item = Coord2D> + '_ {
        (-1..=1)
            .cartesian_product(-1..=1)
            .map(|c| c.into())
            .filter(move |c: &Coord2D| {
                if (c.x == 0 && c.y == 0) || (!diagonal && c.x != 0 && c.y != 0) {
                    return false;
                }
                true
            })
            .map(move |c| c + coord)
            .filter(|c| self.check_coord(c))
    }

    pub fn get_neighbours(
        &self,
        coord: Coord2D,
        diagonal: bool,
    ) -> impl Iterator<Item = (Coord2D, &Item)> + '_ {
        self.get_neighbour_coords(coord, diagonal)
            .map(move |target| (target, self.get(&target).unwrap()))
    }

    pub fn get_neighbour_coords_filtered<'a, P>(
        &'a self,
        coord: Coord2D,
        diagonal: bool,
        predicate: P,
    ) -> impl Iterator<Item = Coord2D> + 'a
    where
        P: Fn(&Coord2D, &Item) -> bool + 'a,
    {
        self.get_neighbours(coord, diagonal)
            .filter(move |(coord, value)| predicate(coord, value))
            .map(|(coord, _)| coord)
    }

    pub fn keys(&self) -> impl Iterator<Item = Coord2D> {
        let width = self.width;
        (0..self.height as i32).flat_map(move |y| (0..width as i32).map(move |x| (x, y).into()))
    }

    pub fn find<P>(&self, predicate: P) -> Option<Coord2D>
    where
        P: FnMut(&(&Coord2D, &Item)) -> bool,
    {
        self.grid.iter().find(predicate).map(|(coord, _)| *coord)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coord2D, &Item)> {
        self.grid.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Coord2D, &mut Item)> {
        self.grid.iter_mut()
    }
}

impl<Item: Clone> Grid<Item> {
    pub fn new_filled(width: u32, height: u32, default: Item) -> Self {
        Self::new_with_initialiser(width, height, || default.clone())
    }
}

impl<Item: Default + Debug> Grid<Item> {
    pub fn new(width: u32, height: u32) -> Self {
        Self::new_with_initialiser(width, height, Default::default)
    }
}

#[allow(dead_code)]
impl<Item: Clone> Grid<Item> {
    pub fn fill(&mut self, value: Item) {
        for coord in self.keys() {
            self.grid.insert(coord, value.clone());
        }
    }
}

impl<Item: Display> Display for Grid<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.do_fmt(f)
    }
}

impl<Item> FromIterator<(Coord2D, Item)> for Grid<Item> {
    fn from_iter<T: IntoIterator<Item = (Coord2D, Item)>>(iter: T) -> Self {
        let mut max = Coord2D::MIN;
        let grid = iter
            .into_iter()
            .inspect(|(coord, _)| {
                assert!(
                    coord.x >= 0 && coord.y >= 0,
                    "Negative coordinates are not allowed"
                );
                max = max.get_max(coord);
            })
            .collect();

        Self {
            grid,
            width: max.x as u32 + 1,
            height: max.y as u32 + 1,
        }
    }
}

impl<Item> IntoIterator for Grid<Item> {
    type Item = (Coord2D, Item);
    type IntoIter = std::collections::hash_map::IntoIter<Coord2D, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter()
    }
}

impl<Item> CommonGrid<Coord2D, Item> for Grid<Item> {
    fn len(&self) -> usize {
        self.grid.len()
    }

    fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }

    fn get(&self, k: &Coord2D) -> Option<&Item> {
        self.grid.get(k)
    }

    fn get_mut(&mut self, k: &Coord2D) -> Option<&mut Item> {
        self.grid.get_mut(k)
    }

    fn set(&mut self, k: Coord2D, v: Item) -> Option<Item> {
        self.grid.insert(k, v)
    }

    fn get_or_set(&mut self, k: &Coord2D, default: Item) -> &Item {
        if !self.grid.contains_key(k) {
            self.set(k.to_owned(), default);
        }
        self.get(k).unwrap()
    }

    fn max_key(&self) -> Coord2D {
        (self.width as i32 - 1, self.height as i32 - 1).into()
    }

    fn min_key(&self) -> Coord2D {
        (0, 0).into()
    }
}

impl<Item> FlatGrid<Coord2D, Item> for Grid<Item> {}
impl<Item: TryFrom<char>> CharGrid<Coord2D, Item> for Grid<Item> {}

impl<Item: Display> DisplayGrid<Coord2D, Item> for Grid<Item> {
    fn get_for_display(&self, key: &Coord2D) -> Option<&dyn Display> {
        self.get(key).map(|i| i as &dyn Display)
    }
}
