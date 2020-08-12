mod direction;
mod index_type;
mod point;

pub mod maths;

pub use point::{Point, PointIndex};
pub use direction::Direction;
pub use index_type::IndexType;

use num::{CheckedMul, FromPrimitive, ToPrimitive, Signed, Integer};

pub type DefaultIx = usize;
pub trait GridNum : Integer + Signed + Clone + Copy + CheckedMul + FromPrimitive + ToPrimitive {}

impl GridNum for i64 {}
impl GridNum for i32 {}
impl GridNum for i16 {}
impl GridNum for i8 {}