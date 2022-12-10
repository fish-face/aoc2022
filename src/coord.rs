use core::ops::Add;
use std::fmt::{Display, Formatter};
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
pub struct Pt<T> (pub T, pub T);

impl<T: Display> Display for Pt<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<T: Add<Output = T>> Add for Pt<T> {
    type Output = Pt<T::Output>;

    fn add(self, other: Self) -> Self::Output
    {
        Pt(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Sub<Output = T> + Add<Output = T>> Sub for Pt<T> {
    type Output = Pt<T>;

    fn sub(self, other: Self) -> Self::Output
    {
        Pt(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Mul<Output = T> + Add + Copy> Pt<T> {
    pub fn scale(self, by: T) -> Self {
        Pt(by * self.0, by * self.1)
    }
}