use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Edge {
    id: String,
    source: String,
    destination: String,
    normalized_weight: f32,
}

impl Edge {
    pub fn from(id: String, source: String, destination: String, normalized_weight: f32) -> Edge {
        return Edge {
            id,
            source,
            destination,
            normalized_weight,
        };
    }
}

pub struct Node {
    pub id: String,
    pub edges: Vec<Edge>,
}

pub struct Graph {
    pub edges: HashMap<String, Edge>,
    pub nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn from(edges: Vec<Edge>) -> Graph {
        let mut edge_map = HashMap::new();
        let mut node_map: HashMap<String, Vec<Edge>> = HashMap::new();

        for edge in edges {
            edge_map.insert(edge.id.clone(), edge.clone());
            add_edge_to_node_map(edge.source.clone(), edge.clone(), &mut node_map);
        }

        let mut nodes: HashMap<String, Node> = HashMap::new();
        for (k, v) in node_map {
            nodes.insert(k.clone(), Node { id: k.clone(), edges: v.to_vec() });
        }

        Graph { nodes, edges: edge_map }
    }
}

fn add_edge_to_node_map(id: String, edge: Edge, node_map: &mut HashMap<String, Vec<Edge>>) {
    let mut vec = node_map.get(id.as_str()).or(Some(&Vec::<Edge>::new())).unwrap().to_vec();
    vec.push(edge);
    node_map.insert(id.clone(), vec);
}


#[test]
fn edge_from_should_construct_edge() {
    let edge = Edge::from("1".to_string(), "2".to_string(), "3".to_string(), 0.5);

    assert_eq!("1", edge.id);
    assert_eq!("2", edge.source);
    assert_eq!("3", edge.destination);
    assert_eq!(0.5, edge.normalized_weight);
}