mod direction;
mod index_type;
mod point;

pub mod maths;

pub use point::{Point, PointIndex};
pub use direction::Direction;
pub use index_type::IndexType;

use num::{Num, CheckedMul, FromPrimitive, ToPrimitive, Signed, Integer};
use std::fmt::Display;

pub type DefaultIx = usize;
pub trait GridNum : Integer + Signed + Clone + Copy + CheckedMul + FromPrimitive + ToPrimitive + Display {}
pub trait GeoNum : Num + Signed + Clone + Copy + FromPrimitive + ToPrimitive + PartialOrd + Display {}

impl GridNum for i64 {}
impl GridNum for i32 {}
impl GridNum for i16 {}
impl GridNum for i8 {}

impl GeoNum for f64 {}
impl GeoNum for f32 {}