// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use crate::{CommonGrid, Coord2D, Coordinate, FlatGrid};
use ansi_term::{Color, Style};
use lazy_static::lazy_static;
use std::cmp;
use std::collections::BinaryHeap;
use std::fmt::Display;

const _COLOURS: [(Color, Color); 26] = [
    (Color::Black, Color::White),
    (Color::Fixed(232), Color::White),
    (Color::Fixed(233), Color::White),
    (Color::Fixed(234), Color::White),
    (Color::Fixed(235), Color::White),
    (Color::Fixed(236), Color::White),
    (Color::Fixed(237), Color::White),
    (Color::Fixed(238), Color::White),
    (Color::Fixed(239), Color::White),
    (Color::Fixed(240), Color::White),
    (Color::Fixed(241), Color::White),
    (Color::Fixed(242), Color::White),
    (Color::Fixed(243), Color::White),
    (Color::Fixed(244), Color::Black),
    (Color::Fixed(245), Color::Black),
    (Color::Fixed(246), Color::Black),
    (Color::Fixed(247), Color::Black),
    (Color::Fixed(248), Color::Black),
    (Color::Fixed(249), Color::Black),
    (Color::Fixed(250), Color::Black),
    (Color::Fixed(251), Color::Black),
    (Color::Fixed(252), Color::Black),
    (Color::Fixed(253), Color::Black),
    (Color::Fixed(254), Color::Black),
    (Color::Fixed(255), Color::Black),
    (Color::White, Color::Black),
];

lazy_static! {
    static ref COLOURS: [Style; 26] = _COLOURS.map(|(b, c)| c.on(b));
}

#[derive(Debug)]
enum StepHeight {
    Start,
    End,
    Step(u8),
}

impl StepHeight {
    fn new(step: char) -> Self {
        assert!(step.is_ascii_alphabetic());
        match step {
            'S' => Self::Start,
            'E' => Self::End,
            _ => {
                assert!(step.is_ascii_lowercase());
                Self::Step((step as u32 - 'a' as u32) as u8)
            }
        }
    }

    fn get_value(&self) -> u8 {
        match self {
            Self::Start => 0,
            Self::End => 25,
            Self::Step(h) => *h,
        }
    }

    fn can_move_to(&self, other: &Self) -> bool {
        other.get_value() <= (self.get_value() + 1)
    }
}

struct Step {
    height: StepHeight,
    cost: i32,
    parent: Option<Coord2D>,
    visited: bool,
}

impl Step {
    fn new(step: char) -> Self {
        Step {
            height: StepHeight::new(step),
            cost: i32::MAX,
            parent: None,
            visited: false,
        }
    }

    fn is_start(&self) -> bool {
        matches!(self.height, StepHeight::Start)
    }

    fn is_start_part_2(&self) -> bool {
        matches!(self.height, StepHeight::Start | StepHeight::Step(0))
    }

    fn is_end(&self) -> bool {
        matches!(self.height, StepHeight::End)
    }

    fn can_move_to(&self, other: &Self) -> bool {
        self.height.can_move_to(&other.height)
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style = match self.height {
            StepHeight::Start => COLOURS[0].bold(),
            StepHeight::End => COLOURS[25].bold(),
            StepHeight::Step(h) => COLOURS[h as usize],
        };
        let char = match self.height {
            StepHeight::Start => 'S',
            StepHeight::End => 'E',
            StepHeight::Step(h) => char::from_u32('a' as u32 + h as u32).unwrap(),
        };
        let style = match self.visited {
            true => style.fg(Color::Green),
            false => style,
        };
        write!(f, "{}{}{}", style.prefix(), char, style.suffix())
        // write!(f, "{}{}", style.prefix(), char)
    }
}

struct PotentialStep {
    heuristic: i32,
    coord: Coord2D,
}

impl PartialEq for PotentialStep {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic.eq(&other.heuristic)
    }
}

impl Eq for PotentialStep {}

impl Ord for PotentialStep {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Backwards to make the queue ascending
        other.heuristic.cmp(&self.heuristic)
    }
}

impl PartialOrd for PotentialStep {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    grid: crate::Grid<Step>,
    end: Coord2D,
}

impl Grid {
    pub fn new_part_1(data: crate::DataIn) -> Grid {
        let mut data = data.peekable();
        let width = data.peek().unwrap().len();
        let mut ret = Self {
            grid: crate::Grid::new_from_iter(
                data.flat_map(|l| l.chars().map(Step::new).collect::<Vec<_>>()),
                width as i32,
            ),
            end: Default::default(),
        };
        ret.end = ret.get_end();
        let startc = ret.get_start();
        let start = ret.grid.get_mut(&startc).unwrap();
        start.cost = 0;
        ret
    }

    pub fn new_part_2(data: crate::DataIn) -> Grid {
        let mut data = data.peekable();
        let width = data.peek().unwrap().len();
        let mut ret = Self {
            grid: crate::Grid::new_from_iter(
                data.flat_map(|l| l.chars().map(Step::new).collect::<Vec<_>>()),
                width as i32,
            ),
            end: Default::default(),
        };
        ret.end = ret.get_end();
        ret.grid
            .iter_mut()
            .map(|(_, s)| s)
            .filter(|s| s.is_start_part_2())
            .for_each(|s| {
                s.visited = true;
                s.cost = 0
            });
        ret
    }

    fn get_start(&self) -> Coord2D {
        self.grid
            .find(|(_, step)| step.is_start())
            .expect("no start?")
    }

    fn get_starts(&self) -> impl Iterator<Item = (Coord2D, i32)> + '_ {
        self.grid
            .iter()
            .filter(|(_, step)| step.is_start_part_2())
            .map(|(coord, _)| (*coord, self.heuristic(coord)))
    }

    fn get_end(&self) -> Coord2D {
        self.grid.find(|(_, step)| step.is_end()).expect("no end?")
    }

    fn is_end(&self, coord: Coord2D) -> bool {
        coord == self.end
    }

    fn len(&self) -> usize {
        self.grid.len()
    }

    fn get_cost(&self, coord: Coord2D) -> i32 {
        self.get(&coord).unwrap().cost
    }

    fn get(&self, k: &Coord2D) -> Option<&Step> {
        self.grid.get(k)
    }

    fn get_mut(&mut self, k: &Coord2D) -> Option<&mut Step> {
        self.grid.get_mut(k)
    }

    fn get_neighbours(&self, coord: Coord2D) -> impl Iterator<Item = Coord2D> + '_ {
        let step = self.get(&coord).unwrap();
        self.grid
            .get_neighbour_coords(coord, false)
            .filter(|coord| step.can_move_to(self.get(coord).unwrap()))
    }

    fn heuristic(&self, coord: &Coord2D) -> i32 {
        self.end.distance(coord)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.fmt(f)
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::new_part_1(data);

    println!("{}", grid);

    let mut queue = BinaryHeap::with_capacity(grid.len());
    queue.push(PotentialStep {
        heuristic: 0,
        coord: grid.get_start(),
    });

    while let Some(PotentialStep { coord, .. }) = queue.pop() {
        if grid.is_end(coord) {
            return Ok(grid.get_cost(coord).to_string());
        }
        let cost = grid.get_cost(coord) + 1;
        let neighbours: Vec<_> = grid.get_neighbours(coord).collect();
        for neighbour_c in neighbours {
            let heuristic = grid.heuristic(&neighbour_c);
            let neighbour = grid.get_mut(&neighbour_c).unwrap();
            if neighbour.cost <= cost {
                continue;
            }
            neighbour.cost = cost;
            neighbour.parent = Some(coord);
            neighbour.visited = true;
            queue.push(PotentialStep {
                heuristic: cost + heuristic,
                coord: neighbour_c,
            })
        }

        // println!("{}", grid);
    }

    panic!("no path to exit");
}

pub fn part_2(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut grid = Grid::new_part_2(data);

    println!("{}", grid);

    let mut queue = BinaryHeap::with_capacity(grid.len());
    for (coord, heuristic) in grid.get_starts() {
        queue.push(PotentialStep { heuristic, coord });
    }

    while let Some(PotentialStep { coord, .. }) = queue.pop() {
        if grid.is_end(coord) {
            return Ok(grid.get_cost(coord).to_string());
        }
        let cost = grid.get_cost(coord) + 1;
        let neighbours: Vec<_> = grid.get_neighbours(coord).collect();
        for neighbour_c in neighbours {
            let heuristic = grid.heuristic(&neighbour_c);
            let neighbour = grid.get_mut(&neighbour_c).unwrap();
            if neighbour.cost <= cost {
                continue;
            }
            neighbour.cost = cost;
            neighbour.parent = Some(coord);
            neighbour.visited = true;
            queue.push(PotentialStep {
                heuristic: cost + heuristic,
                coord: neighbour_c,
            })
        }

        // println!("{}", grid);
    }

    panic!("no path to exit");
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "12",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: Some(crate::AoCPart {
        main: part_2,
        example: part_2
    }),
});
