use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

use crate::{AoCError, AoCResult, CommonGrid, Coord2D, Direction, InfGrid};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
enum Shape {
    Horizontal,
    Plus,
    Angle,
    Vertical,
    Square,
}

impl Shape {
    fn to_coords(self) -> Vec<Coord2D> {
        match self {
            Self::Horizontal => vec![
                Coord2D { x: 0, y: 0 },
                Coord2D { x: 1, y: 0 },
                Coord2D { x: 2, y: 0 },
                Coord2D { x: 3, y: 0 },
            ],
            Self::Plus => vec![
                Coord2D { x: 1, y: 0 },
                Coord2D { x: 1, y: 1 },
                Coord2D { x: 2, y: 1 },
                Coord2D { x: 0, y: 1 },
                Coord2D { x: 1, y: 2 },
            ],
            Self::Angle => vec![
                Coord2D { x: 0, y: 0 },
                Coord2D { x: 1, y: 0 },
                Coord2D { x: 2, y: 0 },
                Coord2D { x: 2, y: 1 },
                Coord2D { x: 2, y: 2 },
            ],
            Self::Vertical => vec![
                Coord2D { x: 0, y: 0 },
                Coord2D { x: 0, y: 1 },
                Coord2D { x: 0, y: 2 },
                Coord2D { x: 0, y: 3 },
            ],
            Self::Square => vec![
                Coord2D { x: 0, y: 0 },
                Coord2D { x: 0, y: 1 },
                Coord2D { x: 1, y: 0 },
                Coord2D { x: 1, y: 1 },
            ],
        }
    }

    fn to_next(self) -> Self {
        match self {
            Self::Horizontal => Self::Plus,
            Self::Plus => Self::Angle,
            Self::Angle => Self::Vertical,
            Self::Vertical => Self::Square,
            Self::Square => Self::Horizontal,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum GridState {
    Rock,
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => '█',
        }
        .fmt(f)
    }
}

fn check_collision(grid: &InfGrid<GridState>, pos: Coord2D, shape: Shape) -> bool {
    for mut coord in shape.to_coords() {
        coord += pos;
        match grid.get(&coord) {
            Some(GridState::Rock) => {
                return true;
            }
            _ => {
                if coord.x < 0 || coord.y < 0 || coord.x >= 7 {
                    return true;
                }
            }
        }
    }
    false
}

fn drop_shape(
    floor: i32,
    shape: Shape,
    grid: &InfGrid<GridState>,
    instructions: &mut dyn Iterator<Item = Direction>,
) -> Coord2D {
    // We're working backwards here
    const DOWN: Coord2D = Direction::North.to_coord();

    let mut pos: Coord2D =
        Coord2D::from((2, floor + 3)) + instructions.next().unwrap().to_coord() + DOWN;
    let mut ret = pos;
    loop {
        let instr = instructions.next().unwrap();
        pos += instr.to_coord();
        if check_collision(grid, pos, shape) {
            pos = ret;
        } else {
            ret = pos;
        }
        pos += DOWN;
        if check_collision(grid, pos, shape) {
            return ret;
        } else {
            ret = pos;
        }
    }
}

pub fn main(mut data: crate::DataIn) -> AoCResult<String> {
    let instructions: Vec<Direction> = data
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '>' => Ok(Direction::East),
            '<' => Ok(Direction::West),
            _ => Err(AoCError::new_from_char(c)),
        })
        .try_collect()?;

    let mut instructions = instructions.into_iter().cycle();
    let mut grid: InfGrid<GridState> = InfGrid::new();
    let mut floor = 0;
    let mut shape = Shape::Horizontal;

    let mut heights = Vec::with_capacity(300);

    for _ in 0..2000 {
        let dropped = drop_shape(floor, shape, &grid, &mut instructions);

        for coord in shape.to_coords().into_iter() {
            grid.set(dropped + coord, GridState::Rock);
        }
        let newfloor = grid.iter().map(|(coord, _)| coord.y).max().unwrap() + 1;
        heights.push((newfloor - floor) as u64);
        floor = newfloor;
        shape = shape.to_next();
    }

    let mut seen = HashSet::new();

    let (end_of_loop, loop_data) = heights
        .iter()
        .copied()
        .tuple_windows()
        .enumerate()
        .skip(888)
        .find(|(_, window): &(_, (_, _, _, _, _, _, _, _, _, _, _, _))| !seen.insert(*window))
        .unwrap();

    let (start_of_loop, _) = heights
        .iter()
        .copied()
        .tuple_windows()
        .enumerate()
        .skip(888)
        .find(|(_, window): &(_, (_, _, _, _, _, _, _, _, _, _, _, _))| window == &loop_data)
        .unwrap();

    println!("{} -> {}", start_of_loop, end_of_loop);

    let loop_data = &heights[start_of_loop..end_of_loop];
    let loop_len = loop_data.len();
    println!(
        "{:?} {} {}",
        loop_data,
        loop_len,
        loop_data.iter().sum::<u64>()
    );

    let formatter = locale::Numeric::load_user_locale().unwrap();

    // const DOOM: usize = 1_000_000_000_000;
    const DOOM: usize = 2022;

    let doom = DOOM - start_of_loop;

    let mut ret = heights[..start_of_loop].iter().sum::<u64>() as usize;
    println!("{}", formatter.format_int(ret));

    let loop_contents = loop_data.iter().sum::<u64>() as usize;

    let loop_times = doom / loop_len;
    println!("{}", loop_times);

    ret += loop_contents * loop_times;
    println!("{}", formatter.format_int(ret));

    let loop_times = doom % loop_len;
    println!("{}", loop_times);
    ret += loop_data.iter().cycle().take(loop_times).sum::<u64>() as usize;
    println!("{}", formatter.format_int(ret));

    // const CORRECT_ANSWER: usize = 1_514_285_714_288;
    const CORRECT_ANSWER: usize = 3092;

    println!(
        "{} {} {}",
        formatter.format_int(ret),
        formatter.format_int(CORRECT_ANSWER),
        formatter.format_int(ret.abs_diff(CORRECT_ANSWER))
    );

    const CURRENT_WRONG_ANSWER: usize = 3187;
    println!(
        "{} {} {}",
        formatter.format_int(ret),
        formatter.format_int(CURRENT_WRONG_ANSWER),
        formatter.format_int(ret.abs_diff(CURRENT_WRONG_ANSWER))
    );

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2022", "17", main));
