mod direction;
mod index_type;
mod point;

pub mod maths;

pub use point::{Point, PointIndex};
pub use direction::Direction;
pub use index_type::IndexType;

use num::{Num, CheckedMul, FromPrimitive, ToPrimitive, Signed, Integer, Float};
use std::fmt::{Debug, Display};
use std::ops::{SubAssign, AddAssign};

pub type DefaultIx = usize;
pub trait OrdNum: Num + PartialOrd + Clone + Copy + Display + Debug + FromPrimitive + ToPrimitive {}
pub trait GridNum : OrdNum + Integer + Signed + AddAssign + SubAssign + CheckedMul {}
pub trait GeoNum : OrdNum + Float + Signed {}


impl GridNum for i64 {}
impl GridNum for i32 {}
impl GridNum for i8 {}

impl GeoNum for f64 {}
impl GeoNum for f32 {}

impl OrdNum for i64 {}
impl OrdNum for i32 {}
impl OrdNum for i8 {}
impl OrdNum for f64 {}
impl OrdNum for f32 {}
impl OrdNum for u64 {}
impl OrdNum for u32 {}
impl OrdNum for u8 {}
impl OrdNum for usize {}