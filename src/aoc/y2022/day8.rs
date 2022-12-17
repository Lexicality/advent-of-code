use std::collections::HashMap;
use std::fmt::Display;

use ansi_term::{Color, Style};
use lazy_static::lazy_static;

const NUM_SIZES: usize = 10;
const NUM_COLOURS: u8 = 24;
const COLOUR_GAP: f64 = NUM_COLOURS as f64 / (NUM_SIZES - 1) as f64;
const BASE_COLOUR: u8 = 232;
const COLOURS: [u8; NUM_SIZES] = [
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
    static ref HIDDEN_COLOURS: [Style; NUM_SIZES] =
        COLOURS.map(|c| Color::Fixed(c).on(Color::Green));
    static ref VISIBLE_COLOURS: [Style; NUM_SIZES] =
        COLOURS.map(|c| Color::Fixed(c).on(Color::Red));
    static ref MYSTERY_COLOURS: [Style; NUM_SIZES] =
        COLOURS.map(|c| Color::Fixed(c).on(Color::Cyan));
}

enum Visibility {
    FileNotFound,
    Visible,
    Hidden,
}
impl Visibility {
    fn setup(val: bool) -> Visibility {
        match val {
            true => Visibility::Visible,
            false => Visibility::FileNotFound,
        }
    }

    fn done(&self) -> bool {
        match self {
            Visibility::FileNotFound => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn rev(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::EAST => Direction::WEST,
            Direction::SOUTH => Direction::NORTH,
            Direction::WEST => Direction::EAST,
        }
    }

    fn idx(&self) -> usize {
        match self {
            Direction::NORTH => 0,
            Direction::EAST => 1,
            Direction::SOUTH => 2,
            Direction::WEST => 3,
        }
    }
}

type Coord = (u32, u32);

struct Tree {
    // coord: Coord,
    height: u32,
    total_visibility: Visibility,
    cardinal_visibility: [Visibility; 4],
}

impl Tree {
    fn new(size: usize, coord: Coord, height: u32) -> Tree {
        let last = (size - 1) as u32;
        let (x, y) = coord;
        let north = x == 0;
        let east = y == last;
        let south = x == last;
        let west = y == 0;

        Tree {
            // coord,
            height,
            total_visibility: Visibility::setup(north || east || south || west),
            cardinal_visibility: [
                Visibility::setup(north),
                Visibility::setup(east),
                Visibility::setup(south),
                Visibility::setup(west),
            ],
        }
    }

    fn style(&self) -> &'static Style {
        let colours: &[Style; NUM_SIZES] = match self.total_visibility {
            Visibility::FileNotFound => &MYSTERY_COLOURS,
            Visibility::Hidden => &HIDDEN_COLOURS,
            Visibility::Visible => &VISIBLE_COLOURS,
        };
        return &colours[self.height as usize];
    }

    fn done(&self, direction: &Direction) -> bool {
        self.total_visibility.done() || self.cardinal_visibility[direction.idx()].done()
    }

    fn set_vis(&mut self, direction: &Direction, visibility: Visibility) {
        if let Visibility::Visible = visibility {
            self.total_visibility = Visibility::Visible;
        }
        self.cardinal_visibility[direction.idx()] = visibility;
    }

    fn is_visible(&self) -> bool {
        match self.total_visibility {
            Visibility::Visible => true,
            _ => false,
        }
    }
}

struct Forest {
    trees: HashMap<Coord, Tree>,
    size: usize,
}

impl Forest {
    fn new(size: usize, data: &mut dyn Iterator<Item = String>) -> Forest {
        let mut forest = Forest {
            trees: HashMap::with_capacity((size * size) as usize),
            size,
        };
        let mut y = 0;
        for line in data {
            for (x, height) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
                let coord: Coord = (x as u32, y);
                forest.trees.insert(coord, Tree::new(size, coord, height));
            }
            y += 1;
        }
        return forest;
    }

    fn zoop(&self, start: &Coord, direction: &Direction) -> Box<dyn Iterator<Item = Coord>> {
        let (x, y) = *start;
        let limit = self.size as u32;

        let iter: Box<dyn Iterator<Item = Coord>> = match direction {
            Direction::NORTH => Box::new((0..y).rev().map(move |y| (x, y))),
            Direction::EAST => Box::new((x + 1..limit).map(move |x| (x, y))),
            Direction::SOUTH => Box::new((y + 1..limit).map(move |y| (x, y))),
            Direction::WEST => Box::new((0..x).rev().map(move |x| (x, y))),
        };
        return iter;
    }

    fn check_vis(&self, coord: &Coord, direction: &Direction, tree_height: u32) -> Visibility {
        for o_coord in self.zoop(coord, direction) {
            if self.get_tree(&o_coord).height >= tree_height {
                return Visibility::Hidden;
            }
        }
        return Visibility::Visible;
    }

    fn invisify(&mut self, coord: &Coord, direction: &Direction) {
        for o_coord in self.zoop(&coord, &direction.rev()) {
            self.get_tree_mut(&o_coord).set_vis(
                // ???
                direction,
                Visibility::Hidden,
            );
        }
    }

    fn get_tree(&self, coord: &Coord) -> &Tree {
        self.trees.get(coord).unwrap()
    }
    fn get_tree_mut(&mut self, coord: &Coord) -> &mut Tree {
        self.trees.get_mut(coord).unwrap()
    }

    fn floodify(&mut self, direction: &Direction, coord: Coord) {
        if self.get_tree(&coord).done(&direction) {
            return;
        }
        let tree_height = self.get_tree(&coord).height;

        if tree_height == 9 {
            self.invisify(&coord, &direction);
        }
        let visibility = self.check_vis(&coord, &direction, tree_height);
        self.get_tree_mut(&coord).set_vis(&direction, visibility);
    }

    fn flood(&mut self) {
        let limit = (self.size - 1) as u32;
        for y in 1..limit {
            for x in 1..limit {
                let coord = (x, y);
                for dir in [
                    Direction::NORTH,
                    Direction::EAST,
                    Direction::SOUTH,
                    Direction::WEST,
                ] {
                    self.floodify(&dir, coord.clone());
                }
                let tree = self.get_tree_mut(&coord);
                if let Visibility::FileNotFound = tree.total_visibility {
                    tree.total_visibility = Visibility::Hidden;
                }
            }
        }
    }

    fn count_visible(&self) -> usize {
        self.trees
            .values()
            .map(|tree| tree.is_visible())
            .filter(|v| *v)
            .count()
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let initial_style = Style::new();
        let mut cstyle = &initial_style;
        for y in 0..self.size {
            write!(f, "{}", cstyle.prefix())?;
            for x in 0..self.size {
                let coord = (x as u32, y as u32);
                let tree = self.trees.get(&coord).unwrap();
                let style = tree.style();
                write!(f, "{}{}", cstyle.infix(*style), tree.height)?;
                cstyle = style;
            }
            write!(f, "{}\n", cstyle.suffix())?;
            cstyle = &initial_style;
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
    // forest.flood_north();
    // println!("{}", forest);
    // forest.flood_south();
    println!("{}", forest);
    return format!("{}", forest.count_visible());
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "8",
    func: main,
});
