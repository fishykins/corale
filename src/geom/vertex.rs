use vek::Vec3;
use crate::core::{GridNum, Point};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vertex<T> where T: GridNum {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vertex<T> where T: GridNum {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

impl<T> Point<T> for Vertex<T> where T: GridNum  {
    fn from_vec3(pos: Vec3<T>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            z: pos.z
        }
    }

    fn to_vec3(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
}