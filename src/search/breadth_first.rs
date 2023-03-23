use crate::graph::Graph;
use crate::grid::Grid;
use crate::node::Node;
use crate::path::PathFinding;
use crate::search::probing;
use crate::search::probing::{bi_directional_probe, probe};

pub struct BreadthFirstSearch {}

pub struct BiBreadthFirstSearch {}

impl PathFinding for BreadthFirstSearch {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return probe(source.clone(), target.id, graph, probing::dequeue);
    }

    fn grid(&self, _source: (usize, usize), _target: (usize, usize), _grid: &Grid) -> Graph {
        return Graph::from(Vec::new());
    }
}

impl PathFinding for BiBreadthFirstSearch {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return bi_directional_probe(source, target, graph);
    }

    fn grid(&self, _source: (usize, usize), _target: (usize, usize), _grid: &Grid) -> Graph {
        return Graph::from(Vec::new());
    }
}