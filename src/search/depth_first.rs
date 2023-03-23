use crate::graph::Graph;
use crate::node::Node;
use crate::path::PathFinding;
use crate::search::probing;
use crate::search::probing::probe;

pub struct DepthFirstSearch {}

impl PathFinding for DepthFirstSearch {
    fn execute(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return probe(source.clone(), target.id, graph, probing::pop);
    }
}