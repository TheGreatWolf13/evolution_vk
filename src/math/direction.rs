use crate::math::ivec3::IVec3;
use crate::math::vec3::Vec3;
use crate::math::{MinMax, Vector3};
use enum_iterator::Sequence;
use enum_map::Enum;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum, Sequence)]
#[repr(u8)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum, Sequence)]
#[repr(u8)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum, Sequence)]
#[repr(u8)]
pub enum AxisDirection {
    Negative,
    Positive,
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

    pub fn get_name(&self) -> &'static str {
        match self {
            Direction::North => "north",
            Direction::East => "east",
            Direction::South => "south",
            Direction::West => "west",
            Direction::Up => "up",
            Direction::Down => "down",
        }
    }

    pub fn get_axis(&self) -> Axis {
        match self {
            Direction::North => Axis::Z,
            Direction::East => Axis::X,
            Direction::South => Axis::Z,
            Direction::West => Axis::X,
            Direction::Up => Axis::Y,
            Direction::Down => Axis::Y,
        }
    }

    pub fn get_axis_direction(&self) -> AxisDirection {
        match self {
            Direction::North => AxisDirection::Negative,
            Direction::East => AxisDirection::Positive,
            Direction::South => AxisDirection::Positive,
            Direction::West => AxisDirection::Negative,
            Direction::Up => AxisDirection::Positive,
            Direction::Down => AxisDirection::Negative,
        }
    }

    pub fn get_horizontal_neighbours(&self) -> [Direction; 2] {
        match self {
            Direction::North => [Direction::East, Direction::West],
            Direction::East => [Direction::South, Direction::North],
            Direction::South | Direction::Up | Direction::Down => [Direction::West, Direction::East],
            Direction::West => [Direction::North, Direction::South],
        }
    }

    pub fn get_vertical_neighbours(&self) -> [Direction; 2] {
        match self {
            Direction::North | Direction::East | Direction::South | Direction::West => [Direction::Up, Direction::Down],
            Direction::Up => [Direction::North, Direction::South],
            Direction::Down => [Direction::South, Direction::North],
        }
    }

    pub fn choose(&self, mut a: Vec3, b: Vec3) -> Vec3 {
        *a.get_mut(self.get_axis()) = self.get_axis_direction().reduce(a.get(self.get_axis()), b.get(self.get_axis()));
        a
    }
}

impl Axis {
    pub fn get_offset(&self) -> IVec3 {
        match self {
            Axis::X => IVec3::X,
            Axis::Y => IVec3::Y,
            Axis::Z => IVec3::Z,
        }
    }

    pub fn get_directions(&self) -> [Direction; 2] {
        match self {
            Axis::X => [Direction::West, Direction::East],
            Axis::Y => [Direction::Down, Direction::Up],
            Axis::Z => [Direction::North, Direction::South],
        }
    }
}

impl AxisDirection {
    pub fn get_offset(&self) -> i32 {
        match self {
            AxisDirection::Negative => -1,
            AxisDirection::Positive => 1
        }
    }

    pub fn reduce<T: MinMax>(&self, a: T, b: T) -> T {
        match self {
            AxisDirection::Negative => a.min(b),
            AxisDirection::Positive => a.max(b),
        }
    }
}