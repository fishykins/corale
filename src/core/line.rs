use super::{GeoNum, Vertex};
use vek::Vec3;

pub struct Line<'a, T> where T: GeoNum {
    start: &'a Vertex<T>,
    target: &'a Vertex<T>,
    directed: bool,
}

impl<'a, T> Line<'a, T> where T: GeoNum {
    pub fn new(start: &'a Vertex<T>, target: &'a Vertex<T>) -> Self {
        Self {
            start,
            target,
            directed: false,
        }
    }

    pub fn new_directed(start: &'a Vertex<T>, target: &'a Vertex<T>) -> Self {
        Self {
            start,
            target,
            directed: true,
        }
    }

    pub fn centroid(&self) -> Vertex<T> {
        (*self.start * *self.target) / (T::one() + T::one())
    }

    pub fn length(&self) -> T {
        T::zero()
    }

    pub fn vector(&self) -> Vec3<T> {
        Vec3::zero()
    }
}