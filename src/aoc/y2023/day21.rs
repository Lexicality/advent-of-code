use std::{collections::HashSet, fmt::Display};

use crate::{AoCError, CharGrid, CommonGrid, Coord2D, Grid};

enum GridState {
    Start,
    Garden,
    Rock,
    Occcupied,
}

impl TryFrom<char> for GridState {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '.' => Ok(GridState::Garden),
            '#' => Ok(GridState::Rock),
            _ => Err(AoCError::new_from_char(value)),
        }
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => 'S',
            Self::Garden => '.',
            Self::Rock => '#',
            Self::Occcupied => 'O',
        }
        .fmt(f)
    }
}

fn big_steppe(grid: &Grid<GridState>, steps: HashSet<Coord2D>) -> HashSet<Coord2D> {
    // let ret = HashSet::with_capacity(steps.capacity() * 2);
    steps
        .into_iter()
        .flat_map(|coord| {
            grid.get_neighbours_filtered(coord, false, |_, item| matches!(item, GridState::Garden))
        })
        .collect()
}

pub fn submain(data: crate::DataIn, num_steps: u32) -> crate::AoCResult<String> {
    let mut grid: Grid<GridState> = Grid::new_from_chars(data)?;
    // println!("{grid}");
    let start = grid
        .find(|(_, item)| matches!(item, GridState::Start))
        .expect("Must have a start");
    grid.set(start, GridState::Garden);

    let mut steps = HashSet::new();
    steps.insert(start);
    for _ in 0..num_steps {
        steps = big_steppe(&grid, steps);
    }
    // steps.into_iter().for_each(|c| {
    //     grid.set(c, GridState::Occcupied);
    // });
    // println!("{grid}");

    let ret = steps.len();
    Ok(ret.to_string())
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    submain(data, 64)
}

pub fn main_example(data: crate::DataIn) -> crate::AoCResult<String> {
    submain(data, 6)
}

inventory::submit!(crate::AoCDay {
    year: "2023",
    day: "21",
    func: main,
    example_func: Some(main_example),
});
