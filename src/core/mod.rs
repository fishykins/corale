mod direction;
mod index_type;
mod point;

pub mod maths;

pub use point::{Point, PointIndex};
pub use direction::Direction;
pub use index_type::IndexType;

use num::{Num, CheckedMul, FromPrimitive, ToPrimitive};

pub type DefaultIx = usize;
pub trait GridNum : Num + Clone + Copy + CheckedMul + FromPrimitive + ToPrimitive {}