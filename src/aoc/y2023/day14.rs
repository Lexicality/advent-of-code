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

pub fn main(data: crate::DataIn) -> String {
    let mut grid: Grid<GridState> = Grid::new_from_lines(
        data.map(|line| line.chars().map(|c| c.try_into().unwrap()).collect_vec()),
    );
    println!("{grid}");
    // tilting time
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
    println!("{grid}");
    grid.iter()
        .filter(|(_, value)| matches!(value, GridState::Pebble))
        .map(|(coord, _)| grid.height - coord.y as u32)
        .sum::<u32>()
        .to_string()
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "14",
    func: main,
    example_func: None,
});
