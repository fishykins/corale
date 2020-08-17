use std::error;
use std::fmt;
use vek::Vec3;
use crate::core::GridNum;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GridError<T> where T: GridNum {
    SpaceOccupied(Vec3<T>),
    OutOfBounds,
}

impl<T> fmt::Display for GridError<T> where T: GridNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GridError::SpaceOccupied(ref e) => write!(f, "Position {} is already in the grid", e),
            GridError::OutOfBounds => write!(f, "Given position is out of bounds"),
        }
    }
}

impl<T> error::Error for GridError<T> where T: GridNum  {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            GridError::OutOfBounds => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            GridError::SpaceOccupied(ref _e) => None,
        }
    }
}