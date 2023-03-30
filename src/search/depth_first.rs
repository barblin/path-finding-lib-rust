use crate::graph::Graph;
use crate::grid::{Direction, Grid};
use crate::node::Node;
use crate::path::PathFinding;
use crate::search::probing;
use crate::search::probing::{probe_graph, probe_grid};

pub struct DepthFirstSearch {}

impl PathFinding for DepthFirstSearch {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return probe_graph(source.id, target.id, graph, probing::pop);
    }

    fn grid(&self, source: (usize, usize), target: (usize, usize), grid: &Grid, directions: &[Direction]) -> Graph {
        return probe_grid(source, target, grid, directions, probing::pop);
    }
}