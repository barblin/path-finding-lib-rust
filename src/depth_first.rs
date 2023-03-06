use crate::probing::probe;

use crate::graph::{Graph, Node};
use crate::path::PathFinding;
use crate::probing;

pub struct DepthFirstSearch {}

impl PathFinding for DepthFirstSearch {
    fn execute(&self, source: Node, target: usize, graph: &Graph) -> Graph {
        return probe(source.clone(), target, graph, probing::stack);
    }
}