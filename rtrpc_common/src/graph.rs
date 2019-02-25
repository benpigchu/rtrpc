use std::collections::{hash_map, HashMap};

/// A directed graph represented with an adjacency list,
/// which allowing self loop and parallel edges
#[derive(Debug, PartialEq, Clone)]
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
    pub fn nodes(&self) -> hash_map::Keys<String, Vec<(String, f64)>> {
        self.nodes.keys()
    }
    pub fn edges(&self, node: &str) -> Option<&Vec<(String, f64)>> {
        self.nodes.get(node)
    }
    pub fn contains_node(&self, node: &str) -> bool {
        self.nodes.contains_key(node)
    }
}

impl Into<Vec<(String, String, f64)>> for Graph {
    fn into(self) -> Vec<(String, String, f64)> {
        let mut vec: Vec<(String, String, f64)> = Vec::new();
        for node in self.nodes() {
            for (target, weight) in self.edges(node).unwrap() {
                vec.push((node.clone(), target.clone(), *weight))
            }
        }
        vec
    }
}

impl From<Vec<(String, String, f64)>> for Graph {
    fn from(vec: Vec<(String, String, f64)>) -> Self {
        Self::from_edges(vec)
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

impl IntoEdge for (&str, &str, f64) {
    fn into_edge(self) -> (String, String, f64) {
        let (start, end, weight) = self;
        (String::from(start), String::from(end), weight)
    }
}

impl<T> IntoEdge for &T
where
    T: IntoEdge + Copy,
{
    fn into_edge(self) -> (String, String, f64) {
        (*self).into_edge()
    }
}

#[test]
fn graph_construction() {
    use std::collections::HashSet;
    let graph = Graph::from_edges(&[
        ("a", "b", 1.0),
        ("b", "c", 2.0),
        ("a", "c", 4.0),
        ("d", "c", 3.0),
        ("c", "e", -4.0),
        ("c", "e", 4.0),
    ]);
    use std::iter::FromIterator;
    let nodes: HashSet<&str> = HashSet::from_iter(graph.nodes().map(|s| s.as_str()));
    let expected_nodes: HashSet<&str> = HashSet::from_iter(vec!["a", "b", "c", "d", "e"]);
    assert_eq!(nodes, expected_nodes);
    let edge_comparer = |(end1, weight1): &(&str, f64), (end2, weight2): &(&str, f64)| {
        let end = end1.cmp(end2);
        use std::cmp::Ordering::Equal;
        if let Equal = end {
            weight1.partial_cmp(&weight2).unwrap()
        } else {
            end
        }
    };
    let check_edge = |node: &str, expected: Vec<(&str, f64)>| {
        let mut edges: Vec<(&str, f64)> = graph
            .edges(node)
            .unwrap()
            .into_iter()
            .map(|(end, weight)| (end.as_str(), *weight))
            .collect();
        let mut expected_edges = expected;
        edges.sort_by(edge_comparer);
        expected_edges.sort_by(edge_comparer);
        assert_eq!(edges, expected_edges);
    };
    check_edge("a", vec![("b", 1.0), ("c", 4.0)]);
    check_edge("b", vec![("c", 2.0)]);
    check_edge("c", vec![("e", 4.0), ("e", -4.0)]);
    check_edge("d", vec![("c", 3.0)]);
    check_edge("e", vec![]);
    assert!(graph.contains_node("a"));
    assert!(graph.contains_node("b"));
    assert!(graph.contains_node("c"));
    assert!(graph.contains_node("d"));
    assert!(graph.contains_node("e"));
    let vec: Vec<_> = graph.clone().into();
    assert_eq!(graph, Graph::from(vec));
}

/// An error type indicate that a negative cycle
/// can be reached from the start point
#[derive(Debug, PartialEq)]
pub struct NegativeCycle();
