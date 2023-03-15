use std::collections::HashMap;

use derivative::Derivative;

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

#[derive(Clone)]
pub struct Node {
    pub id: usize,
    pub edges: Vec<Edge>,
}

pub struct Graph {
    pub edges_lookup: HashMap<usize, Edge>,
    pub nodes_lookup: HashMap<usize, Node>,
    pub edges: Vec<Edge>,
    pub node_count: usize,
}

impl Graph {
    pub fn from(edges: Vec<Edge>) -> Graph {
        let mut edge_map = HashMap::new();
        let mut node_map: HashMap<usize, Vec<Edge>> = HashMap::new();
        let mut node_count: usize = 0;

        for edge in &edges {
            edge_map.insert(edge.index.clone(), edge.clone());
            add_edge_to_node_map(edge.source.clone(), edge.destination.clone(), edge.clone(), &mut node_map);
        }

        let mut nodes: HashMap<usize, Node> = HashMap::new();
        for (k, v) in node_map {
            nodes.insert(k.clone(), Node { id: k.clone(), edges: v.to_vec() });
            node_count += 1;
        }

        Graph {
            nodes_lookup: nodes,
            edges_lookup: edge_map,
            edges,
            node_count,
        }
    }

    pub fn from_adjacency_matrix(matrix: &[&[f32]]) -> Graph {
        let mut index: usize = 0;
        let mut vec: Vec<Edge> = Vec::new();
        for (row, array) in matrix.iter().enumerate() {
            for (col, weight) in array.iter().enumerate() {
                if !weight.eq(&(0.0 as f32)) {
                    vec.push(Edge::from(index, row, col, weight.clone()));
                    index += 1;
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
}

fn add_edge_to_node_map(src: usize, dest: usize, edge: Edge, node_map: &mut HashMap<usize, Vec<Edge>>) {
    let mut vec = node_map.get(&src).or(Some(&Vec::<Edge>::new())).unwrap().to_vec();
    vec.push(edge);
    node_map.insert(src.clone(), vec);
    node_map.entry(dest).or_insert_with(|| Vec::new());
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