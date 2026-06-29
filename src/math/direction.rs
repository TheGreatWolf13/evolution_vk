use crate::math::{MinMax, Vector2, Vector3, Vector4};
use enum_iterator::Sequence;
use enum_map::Enum;
use std::ops::Neg;
use subenum::subenum;
use super_seq_macro::seq;

#[subenum(Direction2, Direction3)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum, Sequence)]
#[repr(u8)]
pub enum Direction4 {
    #[subenum(Direction2, Direction3)]
    East,
    #[subenum(Direction2, Direction3)]
    West,
    #[subenum(Direction2, Direction3)]
    North,
    #[subenum(Direction2, Direction3)]
    South,
    #[subenum(Direction3)]
    Up,
    #[subenum(Direction3)]
    Down,
    Ana,
    Kata,
}

#[subenum(Axis2, Axis3)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum, Sequence)]
#[repr(u8)]
pub enum Axis4 {
    #[subenum(Axis2, Axis3)]
    X,
    #[subenum(Axis2, Axis3)]
    Y,
    #[subenum(Axis3)]
    Z,
    W,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum, Sequence)]
#[repr(u8)]
pub enum AxisDirection {
    Negative,
    Positive,
}

impl AxisDirection {
    pub fn get_offset(self) -> i32 {
        match self {
            AxisDirection::Negative => -1,
            AxisDirection::Positive => 1
        }
    }

    pub fn reduce<T: MinMax>(self, a: T, b: T) -> T {
        match self {
            AxisDirection::Negative => a.min(b),
            AxisDirection::Positive => a.max(b),
        }
    }
}

macro_rules! impl_axis_offset {
    ($name:tt, $vector:tt, $($axes:ident)+) => {
        impl $name {
            pub fn get_offset<V: $vector>(self) -> V {
                match self {
                    $(
                        $name::$axes => V::$axes,
                    )+
                }
            }
        }
    };
}

impl_axis_offset!(Axis2, Vector2, X Y);
impl_axis_offset!(Axis3, Vector3, X Y Z);
impl_axis_offset!(Axis4, Vector4, X Y Z W);

macro_rules! impl_axis_get_directions {
    ($name:tt, $direction:tt, $(($axis:ident => $dir1:ident $dir2:ident)),+) => {
        impl $name {
            pub fn get_directions(self) -> [$direction; 2] {
                match self {
                    $(
                        $name::$axis => [$direction::$dir1, $direction::$dir2],
                    )+
                }
            }
        }
    };
}

impl_axis_get_directions!(Axis2, Direction2, (X => West East), (Y => South North));
impl_axis_get_directions!(Axis3, Direction3, (X => West East), (Y => South North), (Z => Down Up));
impl_axis_get_directions!(Axis4, Direction4, (X => West East), (Y => South North), (Z => Down Up), (W => Kata Ana));

macro_rules! impl_direction_opposite {
    ($name:tt, $(($dir:ident $opposite:ident)),+) => {
        impl $name {
            pub fn opposite(self) -> Self {
                match self {
                    $(
                        $name::$dir => $name::$opposite,
                    )+
                }
            }
        }
    };
}

impl_direction_opposite!(Direction2, (East West), (West East), (North South), (South North));
impl_direction_opposite!(Direction3, (East West), (West East), (North South), (South North), (Up Down), (Down Up));
impl_direction_opposite!(Direction4, (East West), (West East), (North South), (South North), (Up Down), (Down Up), (Ana Kata), (Kata Ana));

macro_rules! impl_direction_get_axis {
    ($name:tt, $axis:tt, $(($dir:ident $axes:ident)),+) => {
        impl $name {
            pub fn get_axis(self) -> $axis {
                match self {
                    $(
                        $name::$dir => $axis::$axes,
                    )+
                }
            }
        }
    };
}

impl_direction_get_axis!(Direction2, Axis2, (East X), (West X), (North Y), (South Y));
impl_direction_get_axis!(Direction3, Axis3, (East X), (West X), (North Y), (South Y), (Up Z), (Down Z));
impl_direction_get_axis!(Direction4, Axis4, (East X), (West X), (North Y), (South Y), (Up Z), (Down Z), (Ana W), (Kata W));

macro_rules! impl_direction_get_axis_direction {
    ($name:tt, $(($dir:ident => $axes:ident)),+) => {
        impl $name {
            pub fn get_axis_direction(self) -> AxisDirection {
                match self {
                    $(
                        $name::$dir => AxisDirection::$axes,
                    )+
                }
            }
        }
    };
}

impl_direction_get_axis_direction!(Direction2, (East => Positive), (West => Negative), (North => Positive), (South => Negative));
impl_direction_get_axis_direction!(Direction3, (East => Positive), (West => Negative), (North => Positive), (South => Negative), (Up => Positive), (Down => Negative));
impl_direction_get_axis_direction!(Direction4, (East => Positive), (West => Negative), (North => Positive), (South => Negative), (Up => Positive), (Down => Negative), (Ana => Positive), (Kata => Negative));

macro_rules! impl_direction_get_offset {
    ($name:tt, $vec_name:tt, $(($dir:ident => $sign:tt $vec:ident)),+) => {
        impl $name {
            pub fn get_offset<V: $vec_name + Neg>(self) -> V {
                match self {
                    $(
                        $name::$dir => V::$vec,
                    )+
                }
            }
        }
    };
}

impl_direction_get_offset!(Direction2, Vector2, (East => + X), (West => - X), (North => + Y), (South => - Y));
impl_direction_get_offset!(Direction3, Vector3, (East => + X), (West => - X), (North => + Y), (South => - Y), (Up => + Z), (Down => - Z));
impl_direction_get_offset!(Direction4, Vector4, (East => + X), (West => - X), (North => + Y), (South => - Y), (Up => + Z), (Down => - Z), (Ana => + W), (Kata => - W));

seq!(N in 2..=4 {
    impl Direction~N {
        pub fn choose<V: Vector~N>(self, mut a: V, b: V) -> V {
            *a.get_mut(self.get_axis()) = self.get_axis_direction().reduce(a.get(self.get_axis()), b.get(self.get_axis()));
             a
        }
    }
});

macro_rules! impl_direction_name {
    ($name:tt, $(($dir:ident => $dir_name:literal)),+) => {
        impl $name {
            pub fn get_name(self) -> &'static str {
                match self {
                    $(
                        $name::$dir => $dir_name,
                    )+
                }
            }
        }
    };
}

impl_direction_name!(Direction2, (East => "east"), (West => "west"), (North => "north"), (South => "south"));
impl_direction_name!(Direction3, (East => "east"), (West => "west"), (North => "north"), (South => "south"), (Up => "up"), (Down => "down"));
impl_direction_name!(Direction4, (East => "east"), (West => "west"), (North => "north"), (South => "south"), (Up => "up"), (Down => "down"), (Ana => "ana"), (Kata => "kata"));