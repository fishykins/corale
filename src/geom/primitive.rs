use crate::core::{GridNum, PointIndex};
use super::{Vertex, Face};

pub trait Primitive<T> where T: GridNum {
    /// Getter for verts
    fn verticies(&self) -> &Vec<Vertex<T>>;
    fn faces(&self) -> &Vec<Face>;
    fn vertex(&self, index: PointIndex) -> Option<&Vertex<T>>;
    fn vertex_mut(&mut self, index: PointIndex) -> Option<&mut Vertex<T>>;
}