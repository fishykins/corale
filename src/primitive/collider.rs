use super::Primitive;
use crate::core::{GridNum};

pub trait Collider<T>: Primitive<T> where T: GridNum
{
    fn intersects(&self, other: &dyn Primitive<T>) -> bool;
    fn contains(&self, other: &dyn Primitive<T>) -> bool;
}