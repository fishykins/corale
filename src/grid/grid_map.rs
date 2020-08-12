use crate::mesh::{BoundingBox, BoxCollider, Cube};
use crate::core::{GridNum, PointIndex};
use super::{Grid3D, GridError};
use vek::Vec3;


pub struct GridMap<I, T> where T: GridNum, I: PartialEq + Clone + Default {
    items: Vec<Option<I>>,
    index_cap: usize,
    indicies: Vec<PointIndex>,
    bounds: BoundingBox<T>,
    width: usize,
    depth: usize,
    offset: Vec3<i64>,
    iter_index: usize,
}

impl<I, T> GridMap<I, T> where T: GridNum, I: PartialEq + Clone + Default {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        let width = (max.x - min.x).to_usize().unwrap();
        let depth = (max.z - min.z).to_usize().unwrap();
        let height = (max.y - min.y).to_usize().unwrap();

        let cap = height + (width * width) + (depth * width * depth);

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
            items: vec![None; cap],
            indicies: Vec::new(),
            index_cap: cap,
            offset,
            width,
            depth,
            iter_index: 0,
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
        index.index() <= self.index_cap
    }

    fn index_used(&self, index: PointIndex) -> bool {
        if self.index_valid(index) {
            return self.items[index.index()].is_some() ;
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
            self.items[i.index()] = Some(item);
            self.indicies.push(i);
            return Ok(i);
        }
        Err(GridError::SpaceOccupied)
    }

    fn remove(&mut self, index: PointIndex) -> bool {
        if self.index_valid(index) && self.index_used(index) {
            self.items[index.index()] = None;
            self.indicies.retain(|v| *v != index);
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
        self.indicies.len()
    }

    fn item(&self, index: PointIndex) -> Option<&I> {
        if self.index_valid(index) {
            return self.items[index.index()].as_ref();
        }
        None
    }

    fn item_mut(&mut self, index: PointIndex) -> Option<&mut I> {
        if self.index_valid(index) {
            return self.items[index.index()].as_mut();
        }
        None
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

impl<I, T> Iterator for GridMap<I, T> where I: PartialEq + Clone + Default, T: GridNum {
    type Item = PointIndex;

    fn next(&mut self) -> Option<PointIndex> {
        if self.iter_index < self.indicies.len() {
            self.iter_index += 1;
            return Some(self.indicies[self.iter_index]);
        }
        self.iter_index = 0;
        None
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