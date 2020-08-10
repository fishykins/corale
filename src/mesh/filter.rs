use super::{Mesh};
use crate::core::GridNum;

pub trait Filter<T> where T: GridNum {
    fn apply(&self, mesh: &mut Mesh<T>);
}