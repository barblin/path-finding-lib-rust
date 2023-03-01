use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Edge {
    index: usize,
    source: usize,
    destination: usize,
    normalized_weight: f32,
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
    pub edges: HashMap<usize, Edge>,
    pub nodes: HashMap<usize, Node>,
}

impl Graph {
    pub fn from(edges: Vec<Edge>) -> Graph {
        let mut edge_map = HashMap::new();
        let mut node_map: HashMap<usize, Vec<Edge>> = HashMap::new();

        for edge in edges {
            edge_map.insert(edge.index.clone(), edge.clone());
            add_edge_to_node_map(edge.source.clone(), edge.clone(), &mut node_map);
        }

        let mut nodes: HashMap<usize, Node> = HashMap::new();
        for (k, v) in node_map {
            nodes.insert(k.clone(), Node { id: k.clone(), edges: v.to_vec() });
        }

        Graph { nodes, edges: edge_map }
    }
}

fn add_edge_to_node_map(id: usize, edge: Edge, node_map: &mut HashMap<usize, Vec<Edge>>) {
    let mut vec = node_map.get(&id).or(Some(&Vec::<Edge>::new())).unwrap().to_vec();
    vec.push(edge);
    node_map.insert(id.clone(), vec);
}


#[test]
fn edge_from_should_construct_edge() {
    let edge = Edge::from(1, 2, 3, 0.5);

    assert_eq!(1, edge.index);
    assert_eq!(2, edge.source);
    assert_eq!(3, edge.destination);
    assert_eq!(0.5, edge.normalized_weight);
}