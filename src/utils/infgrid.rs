use std::{collections::HashMap, fmt::Display};

use crate::Coord2D;

#[derive(Debug, Default)]
pub struct InfGrid<Item> {
    pub grid: HashMap<Coord2D, Item>,
    pub min: Coord2D,
    pub max: Coord2D,
}

impl<Item> InfGrid<Item> {
    pub fn new() -> Self {
        InfGrid {
            grid: HashMap::new(),
            min: Coord2D {
                x: i32::MAX,
                y: i32::MAX,
            },
            max: Coord2D {
                x: i32::MIN,
                y: i32::MIN,
            },
        }
    }

    pub fn get(&self, k: &Coord2D) -> Option<&Item> {
        self.grid.get(k)
    }

    pub fn set(&mut self, k: Coord2D, v: Item) {
        self.min = self.min.get_min(&k);
        self.max = self.max.get_max(&k);
        self.grid.insert(k, v);
    }

    pub fn get_or_set(&mut self, k: &Coord2D, default: Item) -> &Item {
        if !self.grid.contains_key(k) {
            self.set(*k, default);
        }
        self.get(k).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coord2D, &Item)> {
        self.grid.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Coord2D, &mut Item)> {
        self.grid.iter_mut()
    }
}

impl<Item: Default> InfGrid<Item> {
    pub fn get_or_set_default(&mut self, k: &Coord2D) -> &Item {
        self.get_or_set(k, Default::default())
    }
}

impl<Item: Display> Display for InfGrid<Item> {
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
