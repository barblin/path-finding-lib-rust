use crate::{path::PathFinding, graph::{Node, Graph}};

pub struct AStar {}

impl PathFinding for AStar {
    fn execute(&self, _source: Node, _target: Node, _graph: &Graph) -> Graph {
        return Graph::from(Vec::new());
    }
}