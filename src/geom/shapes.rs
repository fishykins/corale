use vek::Vec3;
use crate::core::OrdNum;

pub trait Cube<T> where T: OrdNum {
    fn min(&self) -> Vec3<T>;
    fn max(&self) -> Vec3<T>;
}

pub trait Sphere<T> where T: OrdNum {
    fn center(&self) -> Vec3<T>;
    fn radius(&self) -> T;
}

pub trait Cylinder<T> where T: OrdNum {
    fn center(&self) -> Vec3<T>;
    fn radius(&self) -> T;
    fn height(&self) -> T;
}

pub trait Torus<T> where T: OrdNum {
    fn center(&self) -> Vec3<T>;
    fn major(&self) -> T;
    fn minor(&self) -> T;
}