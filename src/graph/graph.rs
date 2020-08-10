use num::{ToPrimitive, FromPrimitive};  //TODO: Remove the need for unsigned types
use crate::primitive::Cuboid;
use crate::core::GridNum;
use vek::Vec3;

pub trait Graph<I, T>: Cuboid<T>
    where 
        I: PartialEq, 
        T: GridNum + ToPrimitive + FromPrimitive
{
    fn in_bounds(&self, pos: Vec3<T>) -> bool;
    fn add(&mut self, item: I, pos: Vec3<T>) -> bool;
    fn remove(&mut self, pos: Vec3<T>) -> bool;
    fn replace(&mut self, item: I, pos: Vec3<T>) -> bool;
    fn set(&mut self, item: I, pos: Vec3<T>) -> bool;
    fn item(&self, pos: Vec3<T>) -> Option<&I>;
    fn position(&self, item: &I) -> Option<Vec3<T>>;
    fn items(&self) -> Vec<(&I, Vec3<T>)>;
    fn len(&self) -> usize;
}