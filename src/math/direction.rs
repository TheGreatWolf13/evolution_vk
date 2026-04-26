use crate::math::ivec3::IVec3;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl Direction {
    #[inline]
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn get_offset(&self) -> IVec3 {
        match self {
            Direction::North => -IVec3::Z,
            Direction::East => IVec3::X,
            Direction::South => IVec3::Z,
            Direction::West => -IVec3::X,
            Direction::Up => IVec3::Y,
            Direction::Down => -IVec3::Y,
        }
    }
}