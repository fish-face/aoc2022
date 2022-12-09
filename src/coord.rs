use core::ops::Add;
use std::ops::{Mul, Sub};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Coord<T: Add> {
    pub x: T,
    pub y: T,
}

// impl<T: Add> Coord<T> {
//     pub fn add(&self, other: &Self) -> Coord<T>
//     {
//         Coord{x: self.x + other.x, y: self.y + other.y}
//     }
// }
//
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct TCoord<T: Add> (pub T, pub T);

impl<T: Add<Output = T>> Add for TCoord<T> {
    type Output = TCoord<T::Output>;

    fn add(self, other: Self) -> Self::Output
    {
        TCoord(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Sub<Output = T> + Add<Output = T>> Sub for TCoord<T> {
    type Output = TCoord<T>;

    fn sub(self, other: Self) -> Self::Output
    {
        TCoord(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Mul<Output = T> + Add + Copy> TCoord<T> {
    pub fn scale(self, by: T) -> Self {
        TCoord(by * self.0, by * self.1)
    }
}