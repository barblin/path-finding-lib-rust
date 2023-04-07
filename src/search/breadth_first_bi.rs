use crate::graph::Graph;
use crate::grid::{Direction, Grid};
use crate::node::Node;
use crate::path::PathFinding;
use crate::search::probing_bi::{probe_graph, probe_grid};

pub struct BiBreadthFirstSearch {}

impl PathFinding for BiBreadthFirstSearch {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return probe_graph(source, target, graph);
    }

    fn grid(&self, source: (usize, usize), target: (usize, usize), grid: &Grid, directions: &[Direction]) -> Graph {
        return probe_grid(source, target, grid, directions);
    }
}