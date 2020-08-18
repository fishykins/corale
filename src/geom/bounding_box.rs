
use crate::core::OrdNum;
use crate::geom::Area;
use crate::core::maths;
use super::{Cube, BoxCollider};
use vek::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox<T> where T: OrdNum {
    min: Vec3<T>,
    max: Vec3<T>,
}

impl<T> BoundingBox<T> where T: OrdNum {
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn width(&self) -> T {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> T {
        self.max.y - self.min.y
    }

    pub fn depth(&self) -> T {
        self.max.z - self.min.z
    }

    pub fn lerp_x(&self, amount: T) -> T {
        maths::lerpc(self.min.x, self.max.x, amount)
    }

    pub fn lerp_y(&self, amount: T) -> T {
        maths::lerpc(self.min.y, self.max.y, amount)
    }

    pub fn lerp_z(&self, amount: T) -> T {
        maths::lerpc(self.min.z, self.max.z, amount)
    }

    pub fn inverse_lerp(&self, pos: Vec3<T>) -> Vec3<T> {
        Vec3::new(
            maths::inverse_lerp(self.min().x, self.max().x, pos.x),
            maths::inverse_lerp(self.min().y, self.max().y, pos.y),
            maths::inverse_lerp(self.min().z, self.max().z, pos.z)
        )
    }
}

impl<T> Cube<T> for BoundingBox<T> where T: OrdNum {
    fn min(&self) -> Vec3<T> {
        self.min
    }
    fn max(&self) -> Vec3<T> {
        self.max
    }
}

impl<T> BoxCollider<T> for BoundingBox<T> where T: OrdNum {
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

impl<T> Area<T> for BoundingBox<T> where T: OrdNum {
    fn area(&self) -> T {
        self.height() * self.width() * self.depth()
    }
}