use vek::Vec3;
use super::{GridNum, IndexType};

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VertexIndex<Ix = crate::core::DefaultIx>(Ix);

#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord)]
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

    pub fn from_vec3(pos: Vec3<T>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            z: pos.z
        }
    }

    pub fn to_vec3(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl<Ix: IndexType> VertexIndex<Ix> {
    #[inline]
    pub fn new(x: usize) -> Self {
        VertexIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self {
        VertexIndex(IndexType::max())
    }
}