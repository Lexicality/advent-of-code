use std::fmt::Display;

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

    // fn draw(&self, pos: Coord2D, mut grid: InfGrid<GridState>, temp: bool) {
    //     for coord in self.to_coords().into_iter() {
    //         grid.set(
    //             pos + coord,
    //             if temp {
    //                 GridState::TempRock
    //             } else {
    //                 GridState::Rock
    //             },
    //         );
    //     }
    //     println!("{grid:·>-#}");
    // }
}

#[derive(Clone, Copy)]
enum GridState {
    // Void,
    Rock,
    // TempRock,
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::Void => f.fill(),
            Self::Rock => '█',
            // Self::TempRock => '▒',
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
    // let mut pos: Coord2D = Coord2D::from((2, floor + 3));
    // shape.draw(pos, grid.clone(), true);
    // pos += instructions.next().unwrap().to_coord();
    // shape.draw(pos, grid.clone(), true);
    // pos += DOWN;
    // shape.draw(pos, grid.clone(), true);
    let mut ret = pos;
    loop {
        let instr = instructions.next().unwrap();
        // println!("Moving {instr}");
        pos += instr.to_coord();
        if check_collision(grid, pos, shape) {
            // println!("Can't slide");
            pos = ret;
        } else {
            ret = pos;
        }
        // shape.draw(pos, grid.clone(), true);
        pos += DOWN;
        if check_collision(grid, pos, shape) {
            // println!("Can't drop");
            return ret;
        } else {
            ret = pos;
        }
        // shape.draw(pos, grid.clone(), true);
    }
}

pub fn main(data: crate::DataIn) -> AoCResult<String> {
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

    for _ in 0..2022 {
        let dropped = drop_shape(floor, shape, &grid, &mut instructions);

        for coord in shape.to_coords().into_iter() {
            grid.set(dropped + coord, GridState::Rock);
        }
        floor = grid.iter().map(|(coord, _)| coord.y).max().unwrap() + 1;
        shape = shape.to_next();
    }

    Ok(floor.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2022",
    day: "17",
    func: main,
    example_func: None,
});
