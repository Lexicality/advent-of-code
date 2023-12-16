use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

use crate::{AoCError, Coord2D, Direction, Grid};

#[derive(Debug)]
enum PipeSegment {
    Ground,
    Horizontal,
    Vertical,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Mystery,
}

impl TryFrom<char> for PipeSegment {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::BendNorthEast),
            'J' => Ok(Self::BendNorthWest),
            '7' => Ok(Self::BendSouthWest),
            'F' => Ok(Self::BendSouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Mystery),
            _ => Err(AoCError::new(format!("Unknown pipe segment '{value}'"))),
        }
    }
}

impl Display for PipeSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipeSegment::Ground => f.fill(),
            Self::Vertical => '║',
            Self::Horizontal => '═',
            Self::BendNorthEast => '╚',
            Self::BendNorthWest => '╝',
            Self::BendSouthEast => '╔',
            Self::BendSouthWest => '╗',
            Self::Mystery => '╳',
        }
        .fmt(f)
    }
}

fn directionate(pos: Coord2D, first: Direction, second: Direction) -> Option<(Coord2D, Coord2D)> {
    Some((pos + first.into(), pos + second.into()))
}

impl PipeSegment {
    fn get_neighbours(&self, pos: Coord2D) -> Option<(Coord2D, Coord2D)> {
        match self {
            Self::Vertical => directionate(pos, Direction::North, Direction::South),
            Self::Horizontal => directionate(pos, Direction::East, Direction::West),
            Self::BendNorthEast => directionate(pos, Direction::North, Direction::East),
            Self::BendNorthWest => directionate(pos, Direction::North, Direction::West),
            Self::BendSouthEast => directionate(pos, Direction::South, Direction::East),
            Self::BendSouthWest => directionate(pos, Direction::South, Direction::West),
            Self::Ground => None,
            Self::Mystery => None,
        }
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let grid: Grid<PipeSegment> = Grid::new_from_lines(data.map(|line| {
        line.chars()
            .map(|c| c.try_into().unwrap())
            .collect::<Vec<_>>()
    }));
    println!("{grid}");
    let start = grid
        .find(|(_, item)| matches!(item, PipeSegment::Mystery))
        .expect("Must have a starting position");

    let mut queue: VecDeque<(u32, Coord2D)> = grid
        .get_neighbours(start, false)
        .filter(|coord| {
            grid.get(coord)
                .unwrap()
                .get_neighbours(*coord)
                .is_some_and(|(a, b)| a == start || b == start)
        })
        .map(|coord| (1, coord))
        .collect();

    let mut seen: HashSet<Coord2D> = HashSet::with_capacity(grid.len());
    seen.insert(start);

    let mut ret = 0;
    while let Some((mut depth, coord)) = queue.pop_front() {
        ret = ret.max(depth);
        seen.insert(coord);
        let item = grid.get(&coord).unwrap();
        // working on the assumption here that the loop will never go out of bounds
        let (a, b) = item.get_neighbours(coord).expect("loop must be contiguous");
        depth += 1;
        if !seen.contains(&a) {
            queue.push_back((depth, a));
        }
        if !seen.contains(&b) {
            queue.push_back((depth, b));
        }
    }

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "10",
    func: main,
    example_func: None,
});
