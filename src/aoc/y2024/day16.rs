use std::fmt::Display;

// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.
use crate::utils::astar::{a_star, AStarProvider};
use crate::{
    symbols, AoCError, AoCResult, CharGrid, CommonGrid, Coord2D, Coordinate, Direction, Grid,
    RotateDirection,
};

type AStarID = (Coord2D, Direction, RotateDirection);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum GridState {
    Empty,
    Wall,
    Start,
    End,
    Movement(Direction),
}

impl TryFrom<char> for GridState {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => return Err(AoCError::new_from_char(value)),
        })
    }
}

impl Display for GridState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Empty => symbols::VOID,
            GridState::Wall => symbols::BLOCK,
            GridState::Start => 'S',
            GridState::End => 'E',
            GridState::Movement(direction) => return format!("{direction:#}").fmt(f),
        }
        .fmt(f)
    }
}

#[derive(Debug)]
struct AStarImpl {
    grid: Grid<GridState>,
    start: Coord2D,
    end: Coord2D,
}

impl AStarImpl {
    fn new_from_chars(data: crate::DataIn) -> AoCResult<Self> {
        let grid = Grid::new_from_chars(data)?;
        let start = grid
            .find(|(_, v)| matches!(v, GridState::Start))
            .ok_or(AoCError::new("must have a start"))?;
        let end = grid
            .find(|(_, v)| matches!(v, GridState::End))
            .ok_or(AoCError::new("must have an end"))?;

        Ok(Self { grid, start, end })
    }

    fn get_start(&self) -> AStarID {
        (self.start, Direction::East, RotateDirection::None)
    }
}

impl Display for AStarImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.fmt(f)
    }
}

impl AStarProvider for AStarImpl {
    type IDType = AStarID;

    fn get_neighbours(&self, id: &Self::IDType) -> Box<dyn Iterator<Item = Self::IDType> + '_> {
        let (coord, direction, rotation) = *id;
        let direction = direction.rotate(rotation);
        Box::new(
            [
                RotateDirection::None,
                RotateDirection::Left,
                RotateDirection::Right,
            ]
            .into_iter()
            .map(move |rotation| {
                (
                    coord + direction.rotate(rotation).to_coord(),
                    direction,
                    rotation,
                )
            })
            .filter(|(coord, _, _)| {
                matches!(
                    self.grid.get(coord),
                    Some(GridState::Empty) | Some(GridState::End)
                )
            }),
        )
    }

    fn heuristic(&self, (coord, _, _): &Self::IDType) -> u64 {
        self.end.distance(coord).try_into().unwrap()
    }

    fn cost(&self, (_, _, rotation): &Self::IDType) -> u64 {
        match rotation {
            RotateDirection::None => 1,
            RotateDirection::Left | RotateDirection::Right => 1001,
            RotateDirection::Backwards => unreachable!(),
        }
    }

    fn is_end(&self, (coord, _, _): &Self::IDType) -> bool {
        coord == &self.end
    }
}

pub fn part_1(data: crate::DataIn) -> crate::AoCResult<String> {
    let provider = AStarImpl::new_from_chars(data)?;
    // why have I done this to myself
    let mut grid = provider.grid.clone();
    // println!("{grid}");

    let start = provider.get_start();
    let res = a_star(provider, start);

    // comedy debug
    let mut ret = 0;
    for (coord, dir, rot) in res.iter() {
        let v = grid.get_mut(coord).unwrap();
        if matches!(v, GridState::Empty) {
            if *rot == RotateDirection::None {
                ret += 1;
            } else {
                ret += 1001;
            }
            *v = GridState::Movement(dir.rotate(*rot));
        }
    }
    ret += 1;
    println!("{grid}");

    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay {
    year: "2024",
    day: "16",
    part_1: crate::AoCPart {
        main: part_1,
        example: part_1
    },
    part_2: None,
});
