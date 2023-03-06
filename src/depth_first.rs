use crate::probing::probe;

use crate::graph::{Graph, Node};
use crate::path::PathFinding;
use crate::probing;

pub struct DepthFirstSearch {}

impl PathFinding for DepthFirstSearch {
    fn execute(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return probe(source.clone(), target.id, graph, probing::stack);
    }
}