mod complex_graph;
mod simple_graph;

use crate::mesh::Cube;
use crate::core::GridNum;

pub use complex_graph::ComplexGraph;
pub use simple_graph::SimpleGraph;

pub trait Graph<I, T>: Cube<T>
    where 
        I: PartialEq, 
        T: GridNum
{

}