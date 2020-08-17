use crate::geom::{BoundingBox, BoxCollider, Cube};
use crate::core::{GridNum, PointIndex};
use super::{Grid3D, GridError, GridObject};
use vek::Vec3;
use std::collections::HashMap;


pub struct GridMap<I, T> where T: GridNum, I: PartialEq + Clone {
    items: HashMap<PointIndex, GridObject<T, I>>,
    bounds: BoundingBox<T>,
    max_index: usize, 
    dimesnsions: [T; 3],
    hash_order: (usize, usize, usize),
    offset: Vec3<T>,
}

impl<I, T> GridMap<I, T> where T: GridNum, I: PartialEq + Clone {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        let width = (max.x - min.x).to_usize().unwrap();
        let depth = (max.z - min.z).to_usize().unwrap();
        let height = (max.y - min.y).to_usize().unwrap();
        let dimesnsions = [T::from_usize(width).unwrap(), T::from_usize(height).unwrap(), T::from_usize(depth).unwrap()];
        let hash_order = get_hash_order(width, height, depth);

        let max_index = height + (width * width) + (depth * width * depth);
        let mut offset = Vec3::zero();
        
        //prevent negative indexing.
        // !No longer nescisary due to the hashmap implimentation, but meh.
        if min.x < T::zero() {
            offset.x = min.x.abs();
        }
        if min.y < T::zero() {
            offset.y = min.y.abs();
        }
        if min.z < T::zero() {
            offset.z = min.z.abs();
        }

        Self {
            bounds: BoundingBox::new(min, max),
            items: HashMap::new(),
            max_index,
            offset,
            dimesnsions,
            hash_order,
        }
    }

    /// creates a gridmap from a boundingbox. Not as safe as "new"!
    pub fn from_boundingbox(bounds: BoundingBox<T>) -> Self {
        let width = bounds.width().to_usize().unwrap();
        let depth = bounds.depth().to_usize().unwrap();
        let height = bounds.height().to_usize().unwrap();
        let dimesnsions = [T::from_usize(width).unwrap(), T::from_usize(height).unwrap(), T::from_usize(depth).unwrap()];
        let hash_order = get_hash_order(width, height, depth);

        let max_index = height + (width * width) + (depth * width * depth);

        let mut offset = Vec3::zero();
        
        if bounds.min().x < T::zero() {
            offset.x = bounds.min().x.abs();
        }
        if bounds.min().y < T::zero() {
            offset.y = bounds.min().y.abs();
        }
        if bounds.min().z < T::zero() {
            offset.z = bounds.min().z.abs();
        }

        Self {
            bounds,
            items: HashMap::new(),
            max_index,
            offset,
            dimesnsions,
            hash_order,
        }
    }

    /// the hashing algorithm used:
    /// largest axis position value * largest axis width * second largest axis size +
    /// second largest axis position value * second largest axis size +
    /// smallest axis position value
    fn hash(&self, pos: Vec3<T>) -> PointIndex {
        let point = [self.offset.x + pos.x, self.offset.y + pos.y, self.offset.z + pos.z];
        let dim = &self.dimesnsions;

        PointIndex::new(
            (
                (point[self.hash_order.0] * dim[self.hash_order.0] * dim[self.hash_order.1]) + 
                (point[self.hash_order.1] * dim[self.hash_order.1]) + 
                point[self.hash_order.2]
            ).to_usize().unwrap()
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

fn get_hash_order(width: usize, height: usize, depth: usize) -> (usize, usize, usize) {
    if width >= depth && width >= height {
        //case 1: width is the biggest
        if height > depth {
            //(width, height, depth)
            (0, 1, 2)
        }
        else {
            //(width, depth, height)
            (0, 2, 1)
        }
    }
    else if depth >= width && depth >= height {
        //case 2: depth is the biggest
        if height > width {
            //(depth, height, width)
            (2, 1, 0)
        }
        else {
            //(depth, width, height)
            (2, 0, 1)
        }
    }
    else {
        //case 3: height is the biggest
        if width > depth {
            //(height, width, depth)
            (1, 0, 2)
        }
        else {
            //(height, depth, width)
            (1, 2, 0)
        }
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