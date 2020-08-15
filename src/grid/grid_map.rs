use crate::geom::{BoundingBox, BoxCollider, Cube};
use crate::core::{GridNum, PointIndex};
use super::{Grid3D, GridError};
use vek::Vec3;
use std::collections::HashMap;


pub struct GridMap<I, T> where T: GridNum, I: PartialEq + Clone + Default {
    items: HashMap<PointIndex, I>,
    bounds: BoundingBox<T>,
    max_index: usize, 
    width: usize,
    depth: usize,
    offset: Vec3<i64>,
}

impl<I, T> GridMap<I, T> where T: GridNum, I: PartialEq + Clone + Default {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        let width = (max.x - min.x).to_usize().unwrap();
        let depth = (max.z - min.z).to_usize().unwrap();
        let height = (max.y - min.y).to_usize().unwrap();

        let max_index = height + (width * width) + (depth * width * depth);
        let mut offset = Vec3::zero();
        
        if min.x < T::zero() {
            offset.x = min.x.abs().to_i64().unwrap();
        }
        if min.y < T::zero() {
            offset.y = min.y.abs().to_i64().unwrap();
        }
        if min.z < T::zero() {
            offset.z = min.z.abs().to_i64().unwrap();
        }

        Self {
            bounds: BoundingBox::new(min, max),
            items: HashMap::new(),
            max_index,
            offset,
            width,
            depth,
        }
    }

    /// creates a gridmap from a boundingbox. Not as safe as "new"!
    pub fn from_boundingbox(bounds: BoundingBox<T>) -> Self {
        let width = bounds.width().to_usize().unwrap();
        let depth = bounds.depth().to_usize().unwrap();
        let height = bounds.height().to_usize().unwrap();

        let max_index = height + (width * width) + (depth * width * depth);

        let mut offset = Vec3::zero();
        
        if bounds.min().x < T::zero() {
            offset.x = bounds.min().x.abs().to_i64().unwrap();
        }
        if bounds.min().y < T::zero() {
            offset.y = bounds.min().y.abs().to_i64().unwrap();
        }
        if bounds.min().z < T::zero() {
            offset.z = bounds.min().z.abs().to_i64().unwrap();
        }

        Self {
            bounds,
            items: HashMap::new(),
            max_index,
            offset,
            width,
            depth,
        }
    }

    fn hash(&self, pos: Vec3<T>) -> PointIndex {
        let x = self.offset.x + pos.x.to_i64().unwrap();
        let y = self.offset.y + pos.y.to_i64().unwrap();
        let z = self.offset.z + pos.z.to_i64().unwrap();
        PointIndex::new(
            (y + (x * self.width as i64) + (z * self.width as i64 * self.depth as i64)) as usize
        ) 
    }

    fn index_valid(&self, index: PointIndex) -> bool {
        index.index() <= self.max_index
    }

    fn index_used(&self, index: PointIndex) -> bool {
        if self.index_valid(index) {
            return self.items.contains_key(&index);
        }
        return false;
    }
}

impl<I, T> Grid3D<I, T> for GridMap<I, T> where T: GridNum, I: PartialEq + Clone + Default {

    fn add(&mut self, item: I, pos: Vec3<T>) -> Result<PointIndex, GridError> {
        if !self.contains_point(pos) {
            return Err(GridError::OutOfBounds);
        }
        let i = self.hash(pos);
        if !self.index_used(i) {
            self.items.insert(i, item);
            return Ok(i);
        }
        Err(GridError::SpaceOccupied)
    }

    fn remove(&mut self, index: PointIndex) -> bool {
        if self.index_valid(index) && self.index_used(index) {
            self.items.remove(&index);
            return true;
        }
        false
    }

    fn index(&self, pos: Vec3<T>) -> Option<PointIndex> {
        let i = self.hash(pos);
        if self.index_used(i) {
            return Some(i)
        }
        None
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn item(&self, index: PointIndex) -> Option<&I> {
        if self.index_valid(index) {
            return self.items.get(&index)
        }
        None
    }

    fn item_mut(&mut self, index: PointIndex) -> Option<&mut I> {
        if self.index_valid(index) {
            return self.items.get_mut(&index)
        }
        None
    }

    fn items(&self) -> Vec<&I> {
        let mut items = Vec::new();
        for (_, item) in self.items.iter() {
            items.push(item);
        }
        items
    }
}

impl<I, T> Cube<T> for GridMap<I, T> where T: GridNum, I: PartialEq + Clone + Default {
    fn min(&self) -> Vec3<T> {
        self.bounds.min()
    }
    fn max(&self) -> Vec3<T> {
        self.bounds.max()
    }
}


impl<I, T> BoxCollider<T> for GridMap<I, T> where T: GridNum, I: PartialEq + Clone + Default {
    fn contains(&self, other: &dyn Cube<T>) -> bool {
        self.bounds.contains(other)
    }

    fn intersects(&self, other: &dyn Cube<T>) -> bool {
        self.bounds.intersects(other)
    }

    fn contains_point(&self, point: Vec3<T>) -> bool {
        self.bounds.contains_point(point)
    }
}

#[test]
fn gridmap_test() {
    let mut grid = GridMap::<f64, i64>::new(Vec3::zero(), Vec3::from(64));

    let pos = Vec3::new(2,4,12);
    let value = 42.;
    let index = grid.add(value, pos).unwrap();

    assert_eq!(index, grid.index(pos).unwrap());
    assert!(grid.remove(index));
    assert!(grid.index(pos).is_none());
}