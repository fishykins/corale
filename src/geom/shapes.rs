use vek::Vec3;
use crate::core::GridNum;

pub trait Cube<T> where T: GridNum {
    fn min(&self) -> Vec3<T>;
    fn max(&self) -> Vec3<T>;
}

pub trait Sphere<T> where T: GridNum {
    fn radius(&self) -> T;
    fn center(&self) -> Vec3<T>;
}

pub trait Cylinder<T> where T: GridNum {
    fn radius(&self) -> T;
    fn center(&self) -> Vec3<T>;
    fn height(&self) -> T;
}

pub trait Triangle<T> where T: GridNum {
    fn a(&self) -> Vec3<T>;
    fn b(&self) -> Vec3<T>;
    fn c(&self) -> Vec3<T>;
}