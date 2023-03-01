use crate::graph::{Edge, Graph};

pub fn minimum_spanning(graph: Graph) -> Graph {
    return Graph { nodes: graph.nodes, edges: graph.edges };
}

#[test]
fn mst_should_return_graph() {
    let edge = Edge::from(1, 1, 1, 0.5);
    let graph = Graph::from(Vec::from([edge]));
    let min_graph = minimum_spanning(graph);

    assert_eq!(1, min_graph.edges.keys().count());
    assert_eq!(1, min_graph.nodes.keys().count());
}

#[test]
fn mst_should_return_graph_with_source_node_having_one_edge() {
    let edge = Edge::from(1, 2, 3, 0.5);
    let graph = Graph::from(Vec::from([edge]));
    let min_graph = minimum_spanning(graph);

    let source_node = min_graph.nodes.get(&2).unwrap();
    assert_eq!(1, source_node.edges.to_vec().len());
    assert!(!min_graph.nodes.contains_key(&3));
}