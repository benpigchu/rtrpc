mod graph;
mod packet;
pub use graph::*;
pub use packet::*;
/// An error type indicate that a negative cycle
/// can be reached from the start point
#[derive(Debug, PartialEq)]
pub struct NegativeCycle();
