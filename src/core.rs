use rtrpc_graph::*;
use std::collections::{HashMap, VecDeque};

/// Calculate the shortest path from start to end with the Bellman–Ford algorithm.
/// Returns `Err(NegativeCycle)` when a negative cycle can be reached from the start.
/// Returns `Ok(None)` when there are no path from start to end.
/// This value is also returned when one of start and end is not in the graph.
/// Returns `Ok(Some(path))` when the shortest path exist and `path` is the vector of nodes on path.
pub fn shortest_path(
    graph: &Graph,
    start: &str,
    end: &str,
) -> Result<Option<Vec<String>>, NegativeCycle> {
    if !graph.contains_node(start) {
        return Ok(None);
    }
    if !graph.contains_node(end) {
        return Ok(None);
    }
    let mut predecessor: HashMap<&str, Option<&str>> = HashMap::new();
    let mut distance: HashMap<&str, f64> = HashMap::new();
    for node in graph.nodes() {
        predecessor.insert(node, None);
        use std::f64;
        distance.insert(node, f64::INFINITY);
    }
    distance.insert(start, 0.0);
    let node_count = predecessor.len();
    for _ in 1..node_count {
        let mut updated = false;
        for node in graph.nodes() {
            for (target, weight) in graph.edges(node).unwrap() {
                if distance.get(target.as_str()).unwrap()
                    > &(distance.get(node.as_str()).unwrap() + weight)
                {
                    distance.insert(target, distance.get(node.as_str()).unwrap() + weight);
                    predecessor.insert(target, Some(node));
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }
    for node in graph.nodes() {
        for (target, weight) in graph.edges(node).unwrap() {
            if distance.get(target.as_str()).unwrap()
                > &(distance.get(node.as_str()).unwrap() + weight)
            {
                return Err(NegativeCycle {});
            }
        }
    }
    let mut path: VecDeque<String> = VecDeque::new();
    let mut current = end;
    while current != start {
        if let Some(node) = predecessor.get(current).unwrap() {
            path.push_front(String::from(current));
            current = node
        } else {
            return Ok(None);
        }
    }
    path.push_front(String::from(current));
    Ok(Some(path.into_iter().collect()))
}

#[test]
fn shortest_path_test() {
    let graph = Graph::from_edges(&[
        ("a", "b", 1.0),
        ("b", "c", 2.0),
        ("a", "c", 4.0),
        ("d", "c", 3.0),
        ("c", "e", -4.0),
        ("c", "e", 4.0),
        ("f", "g", 4.0),
        ("g", "f", -6.0),
        ("f", "h", 2.0),
        ("h", "f", 2.0),
    ]);
    let make_path = |path: Vec<&str>| {
        Ok::<Option<Vec<String>>, NegativeCycle>(Some(
            path.into_iter().map(|s| String::from(s)).collect(),
        ))
    };
    assert_eq!(shortest_path(&graph, "i", "a"), Ok(None));
    assert_eq!(shortest_path(&graph, "a", "i"), Ok(None));
    assert_eq!(shortest_path(&graph, "f", "h"), Err(NegativeCycle {}));
    assert_eq!(shortest_path(&graph, "h", "a"), Err(NegativeCycle {}));
    assert_eq!(shortest_path(&graph, "a", "h"), Ok(None));
    assert_eq!(shortest_path(&graph, "a", "a"), make_path(vec!["a"]));
    assert_eq!(shortest_path(&graph, "a", "b"), make_path(vec!["a", "b"]));
    assert_eq!(
        shortest_path(&graph, "a", "e"),
        make_path(vec!["a", "b", "c", "e"])
    );
}
