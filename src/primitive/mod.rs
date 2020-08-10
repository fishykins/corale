mod cuboid;
mod collider;
pub use collider::Collider;
pub use cuboid::Cuboid;

use crate::core::GridNum;
use crate::mesh::{Vertex, Face};

pub trait Primitive<T> where T: GridNum {
    fn verticies(&self) -> Vec<Vertex<T>>;
    fn faces(&self) -> Vec<Face>;
}