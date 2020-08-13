use super::{Mesh};
use crate::core::GeoNum;

pub trait Filter<T> where T: GeoNum {
    fn apply(&self, mesh: &mut Mesh<T>);
}