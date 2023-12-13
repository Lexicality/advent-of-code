use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

use crate::{Coord2D, Coordinate};

#[derive(Debug)]
pub struct Grid<Item> {
    pub grid: HashMap<Coord2D, Item>,
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
impl<Item> Grid<Item> {
    pub fn new_from_iter<I>(data: I, width: u32) -> Self
    where
        I: Iterator<Item = Item>,
    {
        assert!(width <= (i32::MAX as u32), "grid is too wide!");
        Self::new_from_lines(data.chunks(width as usize).into_iter())
    }

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

    pub fn new_with_initialiser<F: Fn() -> Item>(width: u32, height: u32, init: F) -> Self {
        Self::validate_dimensions(width, height);
        Self {
            grid: (0..=width)
                .zip(0..=height)
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

    pub fn get_neighbours(
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

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.grid.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coord2D, &Item)> {
        self.grid.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Coord2D, &mut Item)> {
        self.grid.iter_mut()
    }

    pub fn get(&self, k: &Coord2D) -> Option<&Item> {
        self.grid.get(k)
    }

    pub fn get_mut(&mut self, k: &Coord2D) -> Option<&mut Item> {
        self.grid.get_mut(k)
    }

    pub fn set(&mut self, k: Coord2D, v: Item) {
        self.grid.insert(k, v);
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
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                self.grid.get(&(x, y).into()).unwrap().fmt(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
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
