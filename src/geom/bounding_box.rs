
use crate::core::GridNum;
use super::{Cube, BoxCollider};
use vek::Vec3;

pub struct BoundingBox<T> where T: GridNum {
    min: Vec3<T>,
    max: Vec3<T>,
}

impl<T> BoundingBox<T> where T: GridNum {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self {
            min,
            max,
        }
    }
}

impl<T> Cube<T> for BoundingBox<T> where T: GridNum {
    fn min(&self) -> Vec3<T> {
        self.min
    }
    fn max(&self) -> Vec3<T> {
        self.max
    }
}

impl<T> BoxCollider<T> for BoundingBox<T> where T: GridNum {
    fn contains(&self, other: &dyn Cube<T>) -> bool {
        let c1 = self.min().x <= other.min().x;
        let c2 = self.max().x >= other.max().x;
        let c3 = self.min().y <= other.min().y;
        let c4 = self.max().y >= other.max().y;
        let c5 = self.min().z <= other.min().z;
        let c6 = self.max().z >= other.max().z;

        c1 && c2 && c3 && c4 && c5 && c6
    }

    fn intersects(&self, other: &dyn Cube<T>) -> bool {
        let c1 = self.min().x > other.max().x;
        let c2 = self.max().x < other.min().x;
        let c3 = self.min().y > other.max().y;
        let c4 = self.max().y < other.min().y;
        let c5 = self.min().z > other.max().z;
        let c6 = self.max().z < other.min().z;

        !c1 && !c2 && !c3 && !c4 && !c5 && !c6
    }

    fn contains_point(&self, point: Vec3<T>) -> bool {
        let c1 = self.min().x <= point.x;
        let c2 = self.max().x >= point.x;
        let c3 = self.min().y <= point.y;
        let c4 = self.max().y >= point.y;
        let c5 = self.min().z <= point.z;
        let c6 = self.max().z >= point.z;

        c1 && c2 && c3 && c4 && c5 && c6
    }
}