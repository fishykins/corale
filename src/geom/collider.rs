use super::{Primitive, shapes::Cube};
use crate::core::{GridNum};
use vek::Vec3;

pub trait Collider<T> where T: GridNum
{
    fn intersects(&self, other: &dyn Primitive<T>) -> bool;
    fn contains(&self, other: &dyn Primitive<T>) -> bool;
}

pub trait BoxCollider<T> where T: GridNum
{
    fn intersects(&self, other: &dyn Cube<T>) -> bool;
    fn contains(&self, other: &dyn Cube<T>) -> bool;
    fn contains_point(&self, pos: Vec3<T>) -> bool;
}