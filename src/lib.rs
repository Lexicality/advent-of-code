use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops;

pub mod aoc;

pub type DataIn<'a> = &'a mut dyn Iterator<Item = String>;
pub type AoCDayFn = fn(DataIn) -> String;

pub struct AoCDay {
    pub year: &'static str,
    pub day: &'static str,
    pub func: AoCDayFn,
}

inventory::collect!(AoCDay);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord2D {
    pub x: i32,
    pub y: i32,
}

impl Coord2D {
    pub fn parse(data: &str) -> Coord2D {
        let (x, y) = data.split_once(',').unwrap();
        Coord2D {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    pub fn distance(&self, other: &Coord2D) -> u32 {
        return (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs();
    }

    pub fn get_max(&self, other: &Coord2D) -> Coord2D {
        Coord2D {
            x: cmp::max(self.x, other.x),
            y: cmp::max(self.y, other.y),
        }
    }

    pub fn get_min(&self, other: &Coord2D) -> Coord2D {
        Coord2D {
            x: cmp::min(self.x, other.x),
            y: cmp::min(self.y, other.y),
        }
    }
}

impl ops::Add for Coord2D {
    type Output = Self;
    fn add(self, rhs: Coord2D) -> Self::Output {
        Coord2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Coord2D {
    type Output = Self;
    fn sub(self, rhs: Coord2D) -> Self::Output {
        Coord2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul for Coord2D {
    type Output = Self;
    fn mul(self, rhs: Coord2D) -> Self::Output {
        Coord2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Neg for Coord2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Coord2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Coord2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl From<(i32, i32)> for Coord2D {
    fn from(tup: (i32, i32)) -> Self {
        Coord2D { x: tup.0, y: tup.1 }
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "North",
                Direction::East => "East",
                Direction::South => "South",
                Direction::West => "West",
            }
        )
    }
}

impl From<Direction> for Coord2D {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => Coord2D { x: 0, y: 1 },
            Direction::East => Coord2D { x: 1, y: 0 },
            Direction::South => Coord2D { x: 0, y: -1 },
            Direction::West => Coord2D { x: -1, y: 0 },
        }
    }
}

#[derive(Debug)]
struct Grid<Item> {
    pub grid: HashMap<Coord2D, Item>,
    pub width: u32,
    pub height: u32,
}

impl<Item> Grid<Item> {
    pub fn new<I>(data: I, width: u32) -> Grid<Item>
    where
        I: Iterator<Item = Item>,
    {
        assert!(width <= (i32::MAX as u32), "grid is too wide!");
        let data = data.peekable();
        let mut grid = Grid::<Item> {
            grid: HashMap::with_capacity(data.size_hint().0),
            width,
            height: 0,
        };
        let mut y = 0;
        for row in &data.chunks(width as usize) {
            for (x, item) in row.enumerate() {
                grid.grid.insert(Coord2D { x: x as i32, y }, item);
            }
            y += 1;
        }
        grid.height = y as u32;

        return grid;
    }

    pub fn check_coord(&self, coord: &Coord2D) -> bool {
        (coord.x >= 0 && coord.y >= 0)
            && ((coord.x as u32) < self.width && (coord.y as u32) < self.height)
    }

    pub fn get_neighbours<'a>(
        &'a self,
        coord: Coord2D,
        diagonal: bool,
    ) -> impl Iterator<Item = Coord2D> + 'a {
        (-1..1)
            .cartesian_product(-1..1)
            .map(|c| c.into())
            .filter(move |c: &Coord2D| {
                if c.x == 0 && c.y == 0 {
                    return false;
                } else if !diagonal && !(c.x == 0 || c.y == 0) {
                    return false;
                }
                return true;
            })
            .map(move |c| c + coord)
            .filter(|c| self.check_coord(&c))
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = Coord2D> + 'a {
        (0..self.height as i32).flat_map(|y| (0..self.width as i32).map(move |x| (x, y).into()))
    }
}

impl<Item: Display> Display for Grid<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                write!(f, "{}", self.grid.get(&(x, y).into()).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
