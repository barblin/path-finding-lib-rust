use crate::graph::{Graph, Node};
use crate::path::PathFinding;
use crate::probing;
use crate::probing::{bi_directional_probe, probe};

pub struct BreadthFirstSearch {}

pub struct BiBreadthFirstSearch {}

impl PathFinding for BreadthFirstSearch {
    fn execute(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return probe(source.clone(), target.id, graph, probing::queue);
    }
}

impl PathFinding for BiBreadthFirstSearch {
    fn execute(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return bi_directional_probe(source, target, graph);
    }
}