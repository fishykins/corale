use crate::mesh::{Primitive, shapes::Cube};
use crate::core::{GridNum};

pub trait Collider<T> where T: GridNum
{
    fn intersects(&self, other: &dyn Primitive<T>) -> bool;
    fn contains(&self, other: &dyn Primitive<T>) -> bool;
}

pub trait BoxCollider<T> where T: GridNum
{
    fn intersects(&self, other: &dyn Cube<T>) -> bool;
    fn contains(&self, other: &dyn Cube<T>) -> bool;
}