use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Edge {
    index: usize,
    pub source: usize,
    pub destination: usize,
    pub normalized_weight: f32,
}

impl Edge {
    pub fn from(index: usize, source: usize, destination: usize, normalized_weight: f32) -> Edge {
        return Edge {
            index,
            source,
            destination,
            normalized_weight,
        };
    }
}

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
        let mut graph_edges = Vec::new();
        let mut node_count: usize = 0;

        for edge in edges {
            edge_map.insert(edge.index.clone(), edge.clone());
            add_edge_to_node_map(edge.source.clone(), edge.clone(), &mut node_map);
            add_edge_to_node_map(edge.destination.clone(), edge.clone(), &mut node_map);
            graph_edges.push(edge);
        }

        let mut nodes: HashMap<usize, Node> = HashMap::new();
        for (k, v) in node_map {
            nodes.insert(k.clone(), Node { id: k.clone(), edges: v.to_vec() });
            node_count += 1;
        }

        Graph { nodes_lookup: nodes, edges_lookup: edge_map, edges: graph_edges, node_count }
    }

    pub fn sorted_by_weight_asc(&self) -> Vec<Edge> {
        let mut sorted_edges = self.edges.clone();
        sorted_edges.sort_by(|edge1, edge2|
            edge1.normalized_weight.total_cmp(&edge2.normalized_weight));
        return sorted_edges;
    }
}

fn add_edge_to_node_map(id: usize, edge: Edge, node_map: &mut HashMap<usize, Vec<Edge>>) {
    let mut vec = node_map.get(&id).or(Some(&Vec::<Edge>::new())).unwrap().to_vec();
    vec.push(edge);
    node_map.insert(id.clone(), vec);
}


#[test]
fn edge_from_should_construct_edge() {
    let edge = Edge::from(0, 2, 3, 0.5);

    assert_eq!(0, edge.index);
    assert_eq!(2, edge.source);
    assert_eq!(3, edge.destination);
    assert_eq!(0.5, edge.normalized_weight);
}

#[test]
fn sorted_by_weight_asc_should_return_sorted_vec() {
    let edge3 = Edge::from(2, 2, 3, 0.3);
    let edge4 = Edge::from(3, 2, 3, 0.7);
    let edge1 = Edge::from(0, 2, 3, 0.5);
    let edge2 = Edge::from(1, 2, 3, 0.2);

    let graph = Graph::from(Vec::from([edge1, edge2, edge3, edge4]));
    let sorted_edges = graph.sorted_by_weight_asc();

    assert_eq!(0.2, sorted_edges[0].normalized_weight);
    assert_eq!(0.3, sorted_edges[1].normalized_weight);
    assert_eq!(0.5, sorted_edges[2].normalized_weight);
    assert_eq!(0.7, sorted_edges[3].normalized_weight);
}