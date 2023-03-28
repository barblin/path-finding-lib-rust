use crate::{graph::Graph, path::PathFinding};
use crate::node::Node;

pub struct HierarchicalAStar {
    pub heuristic: Box<dyn Fn(usize, usize, &Graph) -> f32>,
}

impl PathFinding for HierarchicalAStar {
    fn graph(&self, _source: Node, _target: Node, _graph: &Graph) -> Graph {
        return Graph::from(Vec::new());
    }
}