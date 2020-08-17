use crate::geom::{BoundingBox, BoxCollider, Cube};
use crate::core::{GridNum, PointIndex};
use super::{Grid3D, GridError, GridObject};
use vek::Vec3;
use std::collections::HashMap;


pub struct GridMap<I, T> where T: GridNum, I: PartialEq + Clone {
    items: HashMap<PointIndex, GridObject<T, I>>,
    bounds: BoundingBox<T>,
    max_index: usize, 
    width: usize,
    depth: usize,
    offset: Vec3<i64>,
}

impl<I, T> GridMap<I, T> where T: GridNum, I: PartialEq + Clone {
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

    fn diag_neighbors(&self, pos: Vec3<T>) -> Vec<PointIndex> {
        let min_x = if pos.x <= T::zero() { 0 } else {(pos.x - T::one()).to_usize().unwrap()};
        let max_x = (pos.x + T::one()).to_usize().unwrap();
        let min_y = if pos.y <= T::zero() { 0 } else {(pos.y - T::one()).to_usize().unwrap()};
        let max_y = (pos.y + T::one()).to_usize().unwrap();
        let min_z = if pos.z <= T::zero() { 0 } else {(pos.z - T::one()).to_usize().unwrap()};
        let max_z = (pos.z + T::one()).to_usize().unwrap();

        let mut array = Vec::new();

        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                for z in min_z..max_z + 1 {
                    let p = Vec3::new(x,y,z).map(|i| T::from_usize(i).unwrap());
                    if p == pos {
                        continue;
                    }

                    let item = self.index(p);
                    if let Some(index) = item {
                        array.push(index);
                    }
                }
            }    
        }
        array
    }

    fn cross_neighbors(&self, pos: Vec3<T>) -> Vec<PointIndex> {
        let mut neighbors = vec![ 
            Vec3::new(pos.x + T::one(), pos.y, pos.z),
            Vec3::new(pos.x, pos.y + T::one(), pos.z),
            Vec3::new(pos.x, pos.y, pos.z + T::one()),
        ];

        if pos.x >= T::one() { neighbors.push(Vec3::new(pos.x - T::one(), pos.y, pos.z))};
        if pos.y >= T::one() { neighbors.push(Vec3::new(pos.x, pos.y - T::one(), pos.z))};
        if pos.z >= T::one() { neighbors.push(Vec3::new(pos.x, pos.y, pos.z - T::one()))};

        let mut array = Vec::new();

        for n in neighbors {
            let item = self.index(n);
            if let Some(index) = item {
                array.push(index);
            }
        }
        array
    }
}

impl<I, T> Grid3D<I, T> for GridMap<I, T> where T: GridNum, I: PartialEq + Clone {

    fn add(&mut self, item: I, pos: Vec3<T>) -> Result<PointIndex, GridError<T>> {
        if !self.contains_point(pos) {
            return Err(GridError::OutOfBounds);
        }
        let i = self.hash(pos);
        if !self.index_used(i) {
            self.items.insert(i, GridObject::new(pos, item));
            return Ok(i);
        }
        Err(GridError::SpaceOccupied(pos))
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

    fn item(&self, index: PointIndex) -> Option<&GridObject<T, I>> {
        if self.index_valid(index) {
            return self.items.get(&index)
        }
        None
    }

    fn item_mut(&mut self, index: PointIndex) -> Option<&mut GridObject<T, I>> {
        if self.index_valid(index) {
            return self.items.get_mut(&index)
        }
        None
    }

    fn items(&self) -> Vec<&GridObject<T, I>> {
        let mut items = Vec::new();
        for (_, item) in self.items.iter() {
            items.push(item);
        }
        items
    }

    fn neighbors(&self, pos: Vec3<T>, diagonal: bool) -> Vec<PointIndex> {
        if diagonal {
            return self.diag_neighbors(pos);
        } else {
            return self.cross_neighbors(pos);
        }
    }
}

impl<I, T> Cube<T> for GridMap<I, T> where T: GridNum, I: PartialEq + Clone {
    fn min(&self) -> Vec3<T> {
        self.bounds.min()
    }
    fn max(&self) -> Vec3<T> {
        self.bounds.max()
    }
}


impl<I, T> BoxCollider<T> for GridMap<I, T> where T: GridNum, I: PartialEq + Clone {
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