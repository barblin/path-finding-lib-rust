use crate::graph::{Edge, Graph};

mod path;
mod graph;
mod union_find;

fn main() {
    let edge = Edge::from(1, 2, 3, 0.5);
    let graph = Graph::from(Vec::from([edge]));
    let mst = path::minimum_spanning(graph);

    println!("{}", mst.edges_lookup.contains_key(&1));
}
