use crate::graph::{Edge, Graph};

mod path;
mod graph;

fn main() {
    let edge = Edge::from("1".to_string(), "2".to_string(), "3".to_string(), 0.5);
    let graph = Graph::from(Vec::from([edge]));
    let mst = path::minimum_spanning(graph);

    println!("{}", mst.edges.contains_key("1"))
}
