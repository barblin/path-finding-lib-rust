use std::cmp::min;
use std::collections::HashMap;
use crate::graph::{Graph, minimum_spanning, Node, Edge};

#[test]
fn minimal_spanning_tree_should_return_graph() {
    let graph = Graph {
        nodes: HashMap::from([("1".to_string(), Node { edges: vec![], id: "1".to_string() })]),
        edges: HashMap::from([("1".to_string(), Edge {
            source: "1".to_string(),
            id: "1".to_string(),
            normalized_weight: 0.5,
            destination: "1".to_string(),
        })]),
    };

    let min_graph = minimum_spanning(graph);

    assert_eq!(1, min_graph.edges.keys().count());
    assert_eq!(1, min_graph.nodes.keys().count());
}