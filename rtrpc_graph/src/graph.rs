use std::collections::HashMap;

/// A directed graph represented with an adjacency list,
/// which allowing self loop and parallel edges
pub struct Graph {
    nodes: HashMap<String, Vec<(String, f64)>>,
}

impl Graph {
    pub fn from_edges<I>(iterable: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoEdge,
    {
        let mut nodes: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        for edge in iterable.into_iter() {
            let (start, end, weight) = edge.into_edge();
            if !nodes.contains_key(&start) {
                nodes.insert(start.clone(), Vec::new());
            }
            if !nodes.contains_key(&end) {
                nodes.insert(end.clone(), Vec::new());
            }
            nodes.get_mut(&start).unwrap().push((end, weight));
        }
        Graph { nodes }
    }
}

pub trait IntoEdge {
    fn into_edge(self) -> (String, String, f64);
}

impl IntoEdge for (String, String, f64) {
    fn into_edge(self) -> (String, String, f64) {
        self
    }
}
