use std::fmt::Display;
use std::str::FromStr;

use crate::{AoCError, Coord2D};

// Implementing `ord` is meaningless because directions don't have magnitude, but
// required for various tie-break shenanigans
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
pub enum RotateDirection {
    Left,
    Right,
    Backwards,
}

impl Direction {
    pub const fn rotate(&self, direction: RotateDirection) -> Direction {
        match direction {
            RotateDirection::Left => match self {
                Self::North => Self::West,
                Self::East => Self::North,
                Self::South => Self::East,
                Self::West => Self::South,
            },
            RotateDirection::Right => match self {
                Self::North => Self::East,
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
            },
            RotateDirection::Backwards => match self {
                Self::North => Self::South,
                Self::East => Self::West,
                Self::South => Self::North,
                Self::West => Self::East,
            },
        }
    }

    pub const fn to_coord(self) -> Coord2D {
        match self {
            Direction::North => Coord2D { x: 0, y: -1 },
            Direction::East => Coord2D { x: 1, y: 0 },
            Direction::South => Coord2D { x: 0, y: 1 },
            Direction::West => Coord2D { x: -1, y: 0 },
        }
    }

    pub const fn from_coord(value: Coord2D) -> Self {
        match value {
            Coord2D { x: 0, y: 0 } => panic!("0, 0 has no direction"),
            Coord2D { x: 0, y } if y < 0 => Direction::North,
            Coord2D { x, y: 0 } if x > 0 => Direction::East,
            Coord2D { x: 0, y } if y > 0 => Direction::South,
            Coord2D { x, y: 0 } if x < 0 => Direction::West,
            _ => panic!("Diagonals are unsupported"),
        }
    }
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

impl FromStr for Direction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            let c = s.chars().next().unwrap();
            c.try_into()
        } else {
            match s.to_lowercase().as_str() {
                "north" => Ok(Self::North),
                "east" => Ok(Self::East),
                "south" => Ok(Self::South),
                "west" => Ok(Self::West),
                _ => Err(AoCError::new(format!("Unknown direction {s}"))),
            }
        }
    }
}

impl TryFrom<Coord2D> for Direction {
    type Error = AoCError;

    fn try_from(value: Coord2D) -> Result<Self, Self::Error> {
        match value {
            Coord2D { x, y } if (x == 0) ^ (y == 0) => Ok(Direction::from_coord(value)),
            Coord2D { x: 0, y: 0 } => Err(AoCError::new("0, 0 has no direction")),
            _ => Err(AoCError::new("Diagonals are unsupported")),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'u' | 'n' | '^' => Ok(Direction::North),
            'r' | 'e' | '>' => Ok(Direction::East),
            'd' | 's' | 'v' => Ok(Direction::South),
            'l' | 'w' | '<' => Ok(Direction::West),
            _ => Err(AoCError::new_from_char(value)),
        }
    }
}
