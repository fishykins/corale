mod grid_map;

use crate::geom::Cube;
use crate::core::{GridNum, PointIndex};
use vek::Vec3;

pub use grid_map::GridMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GridError {
    SpaceOccupied,
    OutOfBounds,
}

pub trait Grid3D<I, T>: Cube<T>
    where 
        I: PartialEq, 
        T: GridNum
        
{
    fn add(&mut self, item: I, pos: Vec3<T>) -> Result<PointIndex, GridError>;
    fn remove(&mut self, index: PointIndex) -> bool;

    fn index(&self, pos: Vec3<T>) -> Option<PointIndex>;

    fn len(&self) -> usize;

    fn item(&self, index: PointIndex) -> Option<&I>;
    fn item_mut(&mut self, index: PointIndex) -> Option<&mut I>;
}