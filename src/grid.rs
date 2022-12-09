// use std::iter::{repeat, zip};
use anyhow::{Result};
use thiserror::Error;
// use crate::coord::Coord;

#[derive(Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

#[derive(Debug, Error)]
pub enum GridErr {
    #[error("Index out of bounds")]
    IndexError
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T>
    where T: Default + Clone
    {
        Grid{width, height, data: vec![T::default(); width * height]}
    }
    pub fn from_data(width: usize, height: usize, data: impl Into<Vec<T>>) -> Grid<T> {
        let mut p = Grid{width, height, data: Vec::new()};
        p.data = data.into();
        p
    }

    pub fn at(&self, x: usize, y: usize) -> Result<&T, GridErr> {
        if x >= self.width || y >= self.height {
            return Err(GridErr::IndexError);
        }
        Ok(&self.data[x + y * self.width])
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> Result<&mut T, GridErr> {
        if x >= self.width || y >= self.height {
            return Err(GridErr::IndexError);
        }
        Ok(&mut self.data[x + y * self.width])
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) -> Result<(), GridErr>{
        if x >= self.width || y >= self.height {
            return Err(GridErr::IndexError);
        }
        self.data[x + y * self.width] = val;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.as_slice().chunks_exact(self.width)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.as_mut_slice().chunks_exact_mut(self.width)
    }

    // pub fn indices_by_row(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Coord<usize>>> {
    //     (0..self.height)
    //         .map(|y| (0..self.width)
    //             .map(move |x| Coord{x, y}))
    // }

    pub fn columns(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        (0..self.width)
            .map(|i| self.data.iter().skip(i).step_by(self.width))
    }

    // pub fn columns_mut(&mut self) -> ColumnsMut<T> {
    //     ColumnsMut::new(self)
    // }

    // pub fn columns_mut(&mut self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &mut T>> {
    //     // zip(0..self.width, repeat(self))
    //     (0..self.width)
    //         .map( |i| self.data.iter_mut().skip(i).step_by_mut(self.width))
    // }

    pub fn to_string(&self, sep: Option<&str>) -> String
    where T: ToString
    {
        let sep = sep.unwrap_or(" ");
        self
            .rows()
            .map(|row| row
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(sep)
            ).collect::<Vec<_>>().join("\n")
    }
}

// struct Iter<T> {
//     p: usize,
// }
//
// impl Iterator for Iter<T> {
//     type Item = &T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//
//     }
// }

// struct ColumnsMut<'a, T: 'a> {
//     grid: &'a Grid<T>,
//     x: usize,
//     y: usize,
// }
//
// impl<'a, T> ColumnsMut<'a, T> {
//     pub fn new(grid: &Grid<T>) -> ColumnsMut<T> {
//         ColumnsMut{grid, x: 0, y: 0 }
//         // grid.data.iter_mut()
//     }
// }
//
// impl<'a, T> Iterator for ColumnsMut<'a, T> {
//     type Item = &'a mut T;
//     fn next(&'a mut self) -> Option<&'a mut T> {
//         let val = self.grid.at(self.x, self.y)?;
//         self.y += 1;
//         if self.y == self.grid.height {
//             self.y = 0;
//             self.x += 1;
//         }
//         Some(val)
//     }
// }