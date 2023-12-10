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
    pub fn new_from_iter<I>(data: I, width: u32) -> Grid<Item>
    where
        I: Iterator<Item = Item>,
    {
        assert!(width <= (i32::MAX as u32), "grid is too wide!");
        Self::new_from_lines(data.chunks(width as usize).into_iter())
    }

    pub fn new_from_lines<Iter, Inner>(data: Iter) -> Grid<Item>
    where
        Inner: IntoIterator<Item = Item>,
        Iter: Iterator<Item = Inner>,
    {
        let mut grid = Grid::<Item> {
            grid: data
                .enumerate()
                .flat_map(|(y, inner)| {
                    inner.into_iter().enumerate().map(move |(x, item)| {
                        (
                            Coord2D {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            },
                            item,
                        )
                    })
                })
                .collect(),
            height: 0,
            width: 0,
        };
        let max_key = grid
            .grid
            .keys()
            .copied()
            .reduce(|a, b| a.get_max(&b))
            .expect("must have at least one entry");
        grid.width = max_key.x as u32 + 1;
        grid.height = max_key.y as u32 + 1;

        grid
    }

    fn validate_dimensions(width: u32, height: u32) -> usize {
        assert!(width <= (i32::MAX as u32), "grid is too wide!");
        assert!(height <= (i32::MAX as u32), "grid is too tall!");
        (width as usize)
            .checked_mul(height as usize)
            .expect("Grid is too big!")
    }

    pub fn new_empty(width: u32, height: u32) -> Grid<Item> {
        let size = Grid::<Item>::validate_dimensions(width, height);
        Grid::<Item> {
            grid: HashMap::with_capacity(size),
            width,
            height,
        }
    }

    pub fn check_coord(&self, coord: &Coord2D) -> bool {
        (coord.x >= 0 && coord.y >= 0)
            && ((coord.x as u32) < self.width && (coord.y as u32) < self.height)
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
    pub fn new_filled(width: u32, height: u32, default: Item) -> Grid<Item> {
        Grid::<Item>::validate_dimensions(width, height);
        Grid::<Item> {
            grid: (0..=width)
                .zip(0..=height)
                .map(|(x, y)| {
                    (
                        Coord2D {
                            x: x as i32,
                            y: y as i32,
                        },
                        default.clone(),
                    )
                })
                .collect(),
            width,
            height,
        }
    }
}

impl<Item: Default + Debug> Grid<Item> {
    pub fn new(width: u32, height: u32) -> Grid<Item> {
        Grid::<Item>::validate_dimensions(width, height);
        Grid::<Item> {
            grid: (0..width)
                .cartesian_product(0..height)
                .map(|(x, y)| {
                    (
                        Coord2D {
                            x: x as i32,
                            y: y as i32,
                        },
                        Default::default(),
                    )
                })
                .collect(),
            width,
            height,
        }
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
