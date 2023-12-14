use std::collections::HashSet;
use std::fmt::Display;

use itertools::Itertools;

use crate::{AoCError, Grid};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GridState {
    Air,
    Pebble,
    Obstacle,
}

impl TryFrom<char> for GridState {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Air),
            '#' => Ok(Self::Obstacle),
            'O' => Ok(Self::Pebble),
            _ => Err(AoCError::new(format!("Invalid character {value}"))),
        }
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Air => '·',
            GridState::Obstacle => '█',
            GridState::Pebble => 'O',
        }
        .fmt(f)
    }
}

fn slide_north(grid: &mut Grid<GridState>) {
    for y in 1..grid.height {
        for x in 0..grid.width {
            let coord = (x, y).try_into().unwrap();
            if let Some(GridState::Pebble) = grid.get(&coord) {
                // long slow slide time
                let mut last_good = None;
                for y2 in (0..y).rev() {
                    let coord2 = (x, y2).try_into().unwrap();
                    match grid.get(&coord2).unwrap() {
                        GridState::Air => last_good = Some(coord2),
                        _ => break,
                    }
                }
                if let Some(last_good) = last_good {
                    grid.set(coord, GridState::Air);
                    grid.set(last_good, GridState::Pebble);
                }
            }
        }
    }
}

fn slide_east(grid: &mut Grid<GridState>) {
    for x in (0..grid.width - 1).rev() {
        for y in 0..grid.height {
            let coord = (x, y).try_into().unwrap();
            if let Some(GridState::Pebble) = grid.get(&coord) {
                // long slow slide time
                let mut last_good = None;
                for x2 in x + 1..grid.width {
                    let coord2 = (x2, y).try_into().unwrap();
                    match grid.get(&coord2).unwrap() {
                        GridState::Air => last_good = Some(coord2),
                        _ => break,
                    }
                }
                if let Some(last_good) = last_good {
                    grid.set(coord, GridState::Air);
                    grid.set(last_good, GridState::Pebble);
                }
            }
        }
    }
}

fn slide_south(grid: &mut Grid<GridState>) {
    for y in (0..grid.height - 1).rev() {
        for x in 0..grid.width {
            let coord = (x, y).try_into().unwrap();
            if let Some(GridState::Pebble) = grid.get(&coord) {
                // long slow slide time
                let mut last_good = None;
                for y2 in y + 1..grid.height {
                    let coord2 = (x, y2).try_into().unwrap();
                    match grid.get(&coord2).unwrap() {
                        GridState::Air => last_good = Some(coord2),
                        _ => break,
                    }
                }
                if let Some(last_good) = last_good {
                    grid.set(coord, GridState::Air);
                    grid.set(last_good, GridState::Pebble);
                }
            }
        }
    }
}

fn slide_west(grid: &mut Grid<GridState>) {
    for x in 1..grid.width {
        for y in 0..grid.height {
            let coord = (x, y).try_into().unwrap();
            if let Some(GridState::Pebble) = grid.get(&coord) {
                // long slow slide time
                let mut last_good = None;
                for x2 in (0..x).rev() {
                    let coord2 = (x2, y).try_into().unwrap();
                    match grid.get(&coord2).unwrap() {
                        GridState::Air => last_good = Some(coord2),
                        _ => break,
                    }
                }
                if let Some(last_good) = last_good {
                    grid.set(coord, GridState::Air);
                    grid.set(last_good, GridState::Pebble);
                }
            }
        }
    }
}

fn spin_cycle(grid: &mut Grid<GridState>) {
    slide_north(grid);
    slide_west(grid);
    slide_south(grid);
    slide_east(grid);
}

fn loading(grid: &Grid<GridState>) -> u32 {
    grid.iter()
        .filter(|(_, value)| matches!(value, GridState::Pebble))
        .map(|(coord, _)| grid.height - coord.y as u32)
        .sum::<u32>()
}

pub fn main(data: crate::DataIn) -> String {
    let mut grid: Grid<GridState> = Grid::new_from_lines(
        data.map(|line| line.chars().map(|c| c.try_into().unwrap()).collect_vec()),
    );
    println!("{grid}");

    let mut seen = HashSet::new();

    let loads = (0..300)
        .map(|_| {
            spin_cycle(&mut grid);
            loading(&grid)
        })
        .collect_vec();

    let (end_of_loop, loop_data) = loads
        .iter()
        .copied()
        .tuple_windows()
        .enumerate()
        .find(|(_, window): &(_, (_, _, _, _))| !seen.insert(*window))
        .unwrap();

    let (start_of_loop, _) = loads
        .iter()
        .copied()
        .tuple_windows()
        .enumerate()
        .find(|(_, window): &(_, (_, _, _, _))| window == &loop_data)
        .unwrap();

    println!("{} -> {}", start_of_loop, end_of_loop);

    let loop_data = &loads[start_of_loop..end_of_loop];
    let loop_len = loop_data.len();
    println!("{:?} {}", loop_data, loop_len);

    const DOOM: usize = 1_000_000_000;

    let wat = (DOOM - start_of_loop) % loop_len - 1;

    println!("Maybe??? {wat} {}", loop_data[wat]);

    // println!(
    //     "I have no idea what I'm doing {} {} {}",
    //     loads[window - wat + 1],
    //     loads[window - wat],
    //     loads[window - wat - 1],
    // );

    // println!("wtf {:?}", &loads[0..20]);

    "".to_owned()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "14",
    func: main,
    example_func: None,
});
