use std::collections::HashMap;

pub struct Edge {
    id: String,
    source: String,
    destination: String,
    normalized_weight: f32,
}

struct Node {
    id: String,
    edges: Vec<Edge>,
}

pub struct Graph {
    edges: HashMap<String, Edge>,
    nodes: HashMap<String, Node>,
}

pub fn minimum_spanning(graph: Graph) -> Graph {
    return Graph { nodes: graph.nodes, edges: graph.edges };
}

#[cfg(test)]
mod graph_tests;