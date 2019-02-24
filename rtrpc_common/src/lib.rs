mod graph;
pub use graph::*;

/// An error type indicate that a negative cycle
/// can be reached from the start point
#[derive(Debug, PartialEq)]
pub struct NegativeCycle();
