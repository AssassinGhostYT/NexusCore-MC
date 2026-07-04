use crate::block::cube::axis::Axis;
use crate::block::cube::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Face {
    pub fn direction(self) -> Option<Direction> {
        match self {
            Face::North => Some(Direction::North),
            Face::South => Some(Direction::South),
            Face::West => Some(Direction::West),
            Face::East => Some(Direction::East),
            _ => None,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Face::Down => Face::Up,
            Face::Up => Face::Down,
            Face::North => Face::South,
            Face::South => Face::North,
            Face::West => Face::East,
            Face::East => Face::West,
        }
    }

    pub fn axis(self) -> Axis {
        match self {
            Face::East | Face::West => Axis::X,
            Face::North | Face::South => Axis::Z,
            Face::Up | Face::Down => Axis::Y,
        }
    }
}
