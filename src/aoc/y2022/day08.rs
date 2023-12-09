use std::collections::HashMap;
use std::fmt::Display;

use ansi_term::{Color, Style};
use lazy_static::lazy_static;

use crate::Direction;

const NUM_SIZES: usize = 10;
const BASE_COLOUR: u8 = 238;
const NUM_COLOURS: u8 = 255 - BASE_COLOUR + 1;
const COLOUR_GAP: f64 = NUM_COLOURS as f64 / (NUM_SIZES - 1) as f64;
const _COLOURS: [u8; NUM_SIZES] = [
    BASE_COLOUR,
    BASE_COLOUR + (COLOUR_GAP * 1.0) as u8,
    BASE_COLOUR + (COLOUR_GAP * 2.0) as u8,
    BASE_COLOUR + (COLOUR_GAP * 3.0) as u8,
    BASE_COLOUR + (COLOUR_GAP * 4.0) as u8,
    BASE_COLOUR + (COLOUR_GAP * 5.0) as u8,
    BASE_COLOUR + (COLOUR_GAP * 6.0) as u8,
    BASE_COLOUR + (COLOUR_GAP * 7.0) as u8,
    BASE_COLOUR + (COLOUR_GAP * 8.0) as u8,
    255,
];
lazy_static! {
    static ref COLOURS: [Style; NUM_SIZES] = _COLOURS.map(|c| Color::Fixed(c).on(Color::Black));
}

impl Direction {
    fn idx(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

type Coord = (u32, u32);

struct Tree {
    // coord: Coord,
    height: u32,
    cardinal_visibility: [usize; 4],
}

impl Tree {
    fn new(height: u32) -> Tree {
        Tree {
            // coord,
            height,
            cardinal_visibility: [0, 0, 0, 0],
        }
    }

    fn style(&self) -> &'static Style {
        &COLOURS[self.height as usize]
    }

    fn set_vis(&mut self, direction: &Direction, visibility: usize) {
        self.cardinal_visibility[direction.idx()] = visibility;
    }

    fn get_visibility(&self) -> usize {
        self.cardinal_visibility.iter().product()
    }
}

struct Forest {
    trees: HashMap<Coord, Tree>,
    size: usize,
}

impl Forest {
    fn new(size: usize, data: &mut dyn Iterator<Item = String>) -> Forest {
        let mut forest = Forest {
            trees: HashMap::with_capacity(size * size),
            size,
        };
        for (y, line) in data.enumerate() {
            for (x, height) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
                let coord: Coord = (x as u32, y as u32);
                forest.trees.insert(coord, Tree::new(height));
            }
        }
        forest
    }

    fn zoop(&self, start: &Coord, direction: &Direction) -> Box<dyn Iterator<Item = Coord>> {
        let (x, y) = *start;
        let limit = self.size as u32;

        let iter: Box<dyn Iterator<Item = Coord>> = match direction {
            Direction::North => Box::new((0..y).rev().map(move |y| (x, y))),
            Direction::East => Box::new((x + 1..limit).map(move |x| (x, y))),
            Direction::South => Box::new((y + 1..limit).map(move |y| (x, y))),
            Direction::West => Box::new((0..x).rev().map(move |x| (x, y))),
        };
        iter
    }

    fn check_vis(&self, coord: &Coord, direction: &Direction, tree_height: u32) -> usize {
        let mut vis = 0;
        for o_coord in self.zoop(coord, direction) {
            vis += 1;
            if self.get_tree(&o_coord).height >= tree_height {
                return vis;
            }
        }
        vis
    }

    fn get_tree(&self, coord: &Coord) -> &Tree {
        self.trees.get(coord).unwrap()
    }
    fn get_tree_mut(&mut self, coord: &Coord) -> &mut Tree {
        self.trees.get_mut(coord).unwrap()
    }

    fn floodify(&mut self, direction: &Direction, coord: Coord) {
        let tree_height = self.get_tree(&coord).height;

        let visibility = self.check_vis(&coord, direction, tree_height);
        self.get_tree_mut(&coord).set_vis(direction, visibility);
    }

    fn flood(&mut self) {
        let limit = (self.size - 1) as u32;
        for y in 1..limit {
            for x in 1..limit {
                let coord = (x, y);
                for dir in [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    self.floodify(&dir, coord);
                }
                // let tree = self.get_tree(&coord);
                // if tree.get_visibility() > 2 {
                // println!("{:?}: {:?}", coord, tree.cardinal_visibility);
                // }
            }
            // println!("");
        }
    }

    fn get_visibilist(&self) -> usize {
        self.trees
            .values()
            .map(|tree| tree.get_visibility())
            .max()
            .unwrap()
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                let coord = (x as u32, y as u32);
                let tree = self.trees.get(&coord).unwrap();
                let style = tree.style();
                write!(
                    f,
                    "{}{}{}({:02})",
                    style.prefix(),
                    tree.height,
                    style.suffix(),
                    tree.get_visibility()
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn main(data: &mut dyn Iterator<Item = String>) -> String {
    let mut data = data.peekable();
    let size = data.peek().unwrap().len();
    let mut forest = Forest::new(size, &mut data);
    println!("{}", forest);
    forest.flood();
    println!("{}", forest);
    format!("{}", forest.get_visibilist())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "8",
    func: main,
    example_func: None,
});
