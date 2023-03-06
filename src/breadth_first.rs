use crate::graph::{Graph, Node};
use crate::path::PathFinding;
use crate::probing;
use crate::probing::probe;

pub struct BreadthFirstSearch {}

pub struct BiBreadthFirstSearch {}

impl PathFinding for BreadthFirstSearch {
    fn execute(&self, source: Node, target: usize, graph: &Graph) -> Graph {
        return probe(source.clone(), target, graph, probing::queue);
    }
}

impl PathFinding for BiBreadthFirstSearch {
    fn execute(&self, _source: Node, _target: usize, _graph: &Graph) -> Graph {
        return Graph::from(Vec::new());
    }
}