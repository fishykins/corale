mod grid_map;

use crate::mesh::Cube;
use crate::core::{GridNum, PointIndex};
use vek::Vec3;

pub use grid_map::GridMap;

pub enum GridError {
    SpaceOccupied,
    OutOfBounds,
}

pub trait Grid3D<I, T>: Cube<T> + Iterator
    where 
        I: PartialEq, 
        T: GridNum
        
{
    fn add(&mut self, item: I, pos: Vec3<T>) -> Result<PointIndex, GridError>;
    fn remove(&mut self, index: PointIndex) -> bool;

    fn index(&self, pos: Vec3<T>) -> Option<PointIndex>;

    fn item(&self, index: PointIndex) -> Option<&I>;
    fn item_mut(&mut self, index: PointIndex) -> Option<&mut I>;
}