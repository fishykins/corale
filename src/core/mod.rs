mod mesh;
mod face;
mod vertex;
mod direction;
mod index_type;

pub mod maths;

pub use mesh::Mesh;
pub use face::{Face, FaceIndex};
pub use vertex::{Vertex, VertexIndex};
pub use direction::Direction;
pub use index_type::IndexType;

use num::{Num, CheckedMul, FromPrimitive};

pub type DefaultIx = usize;
pub trait GridNum : Num + Clone + Copy + CheckedMul + FromPrimitive {}