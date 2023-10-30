use std::fmt::Display;

use crate::Coord2D;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => "North",
            Direction::East => "East",
            Direction::South => "South",
            Direction::West => "West",
        }
        .fmt(f)
    }
}

impl TryFrom<Coord2D> for Direction {
    type Error = String;

    fn try_from(value: Coord2D) -> Result<Self, Self::Error> {
        match value {
            Coord2D { x: 0, y: 0 } => Err("0,0 has no direction".to_owned()),
            Coord2D { x: 0, y } if y > 0 => Ok(Direction::North),
            Coord2D { x, y: 0 } if x > 0 => Ok(Direction::East),
            Coord2D { x: 0, y } if y < 0 => Ok(Direction::South),
            Coord2D { x, y: 0 } if x < 0 => Ok(Direction::West),
            _ => Err("Diagonals are unsupported".to_owned()),
        }
    }
}
