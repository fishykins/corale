use crate::mesh::{BoundingBox};
use crate::core::{GridNum, PointIndex};

pub struct SimpleGraph<T> where T: GridNum {
    _grid_array: Vec<Vec<Vec<Option<PointIndex>>>>,
    _bounds: BoundingBox<T>,
}