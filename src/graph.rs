use std::collections::hash_map::Entry;
use std::collections::HashMap;

use derivative::Derivative;

use crate::node::{Node, Vec3};
use crate::union_find::UnionFind;

#[derive(Derivative)]
#[derivative(Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    index: usize,
    pub source: usize,
    pub destination: usize,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    pub weight: f32,
}

impl Edge {
    pub fn from(index: usize, source: usize, destination: usize, weight: f32) -> Edge {
        return Edge {
            index,
            source,
            destination,
            weight,
        };
    }
}

pub struct Graph {
    pub edges_lookup: HashMap<usize, Edge>,
    pub nodes_lookup: HashMap<usize, Node>,
    pub node_position_lookup: Option<HashMap<usize, Vec3>>,
    pub edges: Vec<Edge>,
    pub node_count: usize,
}

impl Graph {
    pub fn from(edges: Vec<Edge>) -> Graph {
        let mut nodes: HashMap<usize, Node> = HashMap::new();
        let edge_map = edges.iter().map(|edge| {
            match nodes.entry(edge.source) {
                Entry::Vacant(entry) => { entry.insert(Node::from(edge.source, vec![edge.clone()])); }
                Entry::Occupied(mut entry) => { entry.get_mut().edges.push(edge.clone()); }
            }

            match nodes.entry(edge.destination) {
                Entry::Vacant(entry) => { entry.insert(Node::from(edge.destination, vec![])); }
                Entry::Occupied(mut _entry) => {}
            }

            return (edge.index, edge.clone());
        }).collect();

        let node_size: usize = nodes.keys().len();

        Graph {
            nodes_lookup: nodes,
            edges_lookup: edge_map,
            node_position_lookup: None,
            edges,
            node_count: node_size,
        }
    }

    pub fn from_adjacency_matrix(matrix: &[&[f32]]) -> Graph {
        let mut vec: Vec<Edge> = Vec::new();
        for (row, array) in matrix.iter().enumerate() {
            for (col, weight) in array.iter().enumerate() {
                if !weight.eq(&(0.0 as f32)) {
                    vec.push(Edge::from(row * array.len() + col, row, col, weight.clone()));
                }
            }
        }

        return Graph::from(vec);
    }

    pub fn sorted_by_weight_asc(&self) -> Vec<Edge> {
        let mut sorted_edges = self.edges.clone();
        sorted_edges.sort_by(|edge1, edge2|
            edge1.weight.total_cmp(&edge2.weight));
        return sorted_edges;
    }

    pub fn offer_positions(&mut self, node_positions: HashMap<usize, Vec3>) {
        self.node_position_lookup = Some(node_positions);
    }

    pub fn verify_positions(&self) {
        return match &self.node_position_lookup {
            None => panic!("You must offer node positions to the graph before using this\
             heuristic. Make sure to provide a Vec3 for every node id."),
            _ => {}
        };
    }

    pub fn position_is_set(&self) -> bool {
        return self.node_position_lookup.is_some();
    }

    pub fn get_position(&self, node_id: &usize) -> &Vec3 {
        match &self.node_position_lookup {
            None => panic!("You must offer node positions to the graph before using this heuristic."),
            Some(positions) => {
                return match positions.get(node_id) {
                    None => panic!("Node position missing for given node id: {node_id}"),
                    Some(position) => position
                };
            }
        };
    }
}

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
        total_cost += edge.weight;
    }

    assert_eq!(0.7142857143, total_cost);
}

#[test]
fn edge_from_should_construct_edge() {
    let edge = Edge::from(0, 2, 3, 0.5);

    assert_eq!(0, edge.index);
    assert_eq!(2, edge.source);
    assert_eq!(3, edge.destination);
    assert_eq!(0.5, edge.weight);
}

#[test]
fn sorted_by_weight_asc_should_return_sorted_vec() {
    let edge3 = Edge::from(2, 2, 3, 0.3);
    let edge4 = Edge::from(3, 2, 3, 0.7);
    let edge1 = Edge::from(0, 2, 3, 0.5);
    let edge2 = Edge::from(1, 2, 3, 0.2);

    let graph = Graph::from(Vec::from([edge1, edge2, edge3, edge4]));
    let sorted_edges = graph.sorted_by_weight_asc();

    assert_eq!(0.2, sorted_edges[0].weight);
    assert_eq!(0.3, sorted_edges[1].weight);
    assert_eq!(0.5, sorted_edges[2].weight);
    assert_eq!(0.7, sorted_edges[3].weight);
}

#[test]
fn create_graph_from_adjacency_matrix() {
    let matrix: &[&[f32]] = &[
        &[0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0],
        &[4.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 11.0, 0.0],
        &[0.0, 8.0, 0.0, 7.0, 0.0, 4.0, 0.0, 0.0, 2.0],
        &[0.0, 0.0, 7.0, 0.0, 9.0, 14.0, 0.0, 0.0, 0.0],
        &[0.0, 0.0, 0.0, 9.0, 0.0, 10.0, 0.0, 0.0, 0.0],
        &[0.0, 0.0, 4.0, 14.0, 10.0, 0.0, 2.0, 0.0, 0.0],
        &[0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 1.0, 6.0],
        &[8.0, 11.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 7.0],
        &[0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 6.0, 7.0, 0.0]
    ];

    let graph = Graph::from_adjacency_matrix(matrix);

    assert_eq!(28, graph.edges.len());
    assert_eq!(2, graph.nodes_lookup.get(&0).unwrap().edges.len());
    assert_eq!(3, graph.nodes_lookup.get(&8).unwrap().edges.len());
    assert_eq!(2.0, graph.nodes_lookup.get(&8).unwrap().edges[0].weight);
}

#[test]
fn create_initial_graph_should_not_have_node_positions() {
    let edge = Edge::from(0, 2, 3, 0.5);
    let graph = Graph::from(Vec::from([edge]));

    assert!(graph.node_position_lookup.is_none());
}

#[test]
fn offer_node_positions_should_set_node_positions() {
    let edge = Edge::from(0, 2, 3, 0.5);
    let mut graph = Graph::from(Vec::from([edge.clone()]));

    let mut node_positions: HashMap<usize, Vec3> = HashMap::new();
    node_positions.insert((&edge).source.clone(), Vec3::from(0.3, 0.2, 0.0));
    node_positions.insert((&edge).destination.clone(), Vec3::from(0.1, 0.5, 0.0));

    graph.offer_positions(node_positions);

    assert!(graph.node_position_lookup.is_some());

    let position_lookup = graph.node_position_lookup.unwrap();
    assert_eq!(0.3, position_lookup.get(&(&edge).source).unwrap().x);
    assert_eq!(0.1, position_lookup.get(&(&edge).destination).unwrap().x);
}