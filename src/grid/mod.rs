mod grid_map;
mod grid_object;

use crate::geom::Cube;
use crate::core::{GridNum, PointIndex};
use vek::Vec3;

pub use grid_map::GridMap;
pub use grid_object::GridObject;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GridError {
    SpaceOccupied,
    OutOfBounds,
}

pub trait Grid3D<I, T>: Cube<T>
    where 
        I: PartialEq + Clone,
        T: GridNum
        
{
    fn add(&mut self, item: I, pos: Vec3<T>) -> Result<PointIndex, GridError>;
    fn remove(&mut self, index: PointIndex) -> bool;

    fn index(&self, pos: Vec3<T>) -> Option<PointIndex>;

    fn len(&self) -> usize;

    fn item(&self, index: PointIndex) -> Option<&GridObject<T, I>>;
    fn item_mut(&mut self, index: PointIndex) -> Option<&mut GridObject<T, I>>;

    fn items(&self) -> Vec<&GridObject<T, I>>;

    fn neighbors(&self, pos: Vec3<T>, diagonal: bool) -> Vec<PointIndex>;
}