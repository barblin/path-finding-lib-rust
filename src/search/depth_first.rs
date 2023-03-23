use crate::graph::Graph;
use crate::grid::Grid;
use crate::node::Node;
use crate::path::PathFinding;
use crate::search::probing;
use crate::search::probing::probe;

pub struct DepthFirstSearch {}

impl PathFinding for DepthFirstSearch {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return probe(source.clone(), target.id, graph, probing::pop);
    }

    fn grid(&self, _source: (usize, usize), _target: (usize, usize), _grid: &Grid) -> Graph {
        return Graph::from(Vec::new());
    }
}