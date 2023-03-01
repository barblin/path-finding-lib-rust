use crate::graph::{Edge, Graph};
use crate::union_find::UnionFind;

pub fn minimum_spanning(graph: Graph) -> Graph {
    let edges = graph.sorted_by_weight_asc();
    let mut union_find = UnionFind::from(graph.node_count);
    let mut min_edges = Vec::new();

    for edge in edges {
        if !union_find.connected(edge.source, edge.destination) {
            union_find.unify(edge.source, edge.destination);
            min_edges.push(edge);
        }
    }

    return Graph::from(min_edges);
}


#[test]
fn mst_should_return_graph() {
    let edge = Edge::from(0, 0, 1, 0.5);
    let graph = Graph::from(Vec::from([edge]));
    let min_graph = minimum_spanning(graph);

    assert_eq!(1, min_graph.edges_lookup.keys().count());
    assert_eq!(2, min_graph.nodes_lookup.keys().count());
}

#[test]
fn mst_should_return_graph_with_source_node_having_one_edge() {
    let edge = Edge::from(0, 0, 1, 0.5);
    let graph = Graph::from(Vec::from([edge]));
    let min_graph = minimum_spanning(graph);

    let source_node = min_graph.nodes_lookup.get(&0).unwrap();
    assert_eq!(1, source_node.edges.to_vec().len());
    assert!(min_graph.nodes_lookup.contains_key(&0));
    assert!(min_graph.nodes_lookup.contains_key(&1));
}

#[test]
fn mst_should_return_minimum_spanning_tree() {
    let edge1 = Edge::from(0, 1, 2, 0.0);
    let edge2 = Edge::from(1, 2, 3, 0.1428571429);
    let edge3 = Edge::from(2, 1, 0, 0.2857142857);
    let edge4 = Edge::from(3, 3, 4, 0.2857142857);
    let edge5 = Edge::from(4, 1, 3, 0.4285714286);
    let edge6 = Edge::from(5, 0, 3, 0.8571428571);
    let edge7 = Edge::from(6, 0, 4, 1.0);


    let graph = Graph::from(Vec::from([edge1, edge2, edge3, edge4, edge5, edge6, edge7]));
    let min_graph = minimum_spanning(graph);

    let mut total_cost: f32 = 0.0;
    for edge in min_graph.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(0.7142857143, total_cost);
}