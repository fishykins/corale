use vek::Vec3;
use super::Collider;
use crate::core::GridNum;

pub trait Cuboid<T>: Collider<T> where T: GridNum {
    fn min(&self) -> Vec3<T>;
    fn max(&self) -> Vec3<T>;
}