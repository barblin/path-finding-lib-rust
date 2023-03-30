use crate::graph::Graph;
use crate::grid::{Direction, Grid};
use crate::node::Node;
use crate::path::PathFinding;
use crate::search::probing;
use crate::search::probing::{bi_directional_probe, probe_graph, probe_grid};

pub struct BreadthFirstSearch {}

pub struct BiBreadthFirstSearch {}

impl PathFinding for BreadthFirstSearch {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return probe_graph(source.id, target.id, graph, probing::dequeue);
    }

    fn grid(&self, source: (usize, usize), target: (usize, usize), grid: &Grid, directions: &[Direction]) -> Graph {
        return probe_grid(source, target, grid, directions, probing::dequeue);
    }
}

impl PathFinding for BiBreadthFirstSearch {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return bi_directional_probe(source, target, graph);
    }

    fn grid(&self, _source: (usize, usize), _target: (usize, usize), _grid: &Grid, _directions: &[Direction]) -> Graph {
        return Graph::from(Vec::new());
    }
}