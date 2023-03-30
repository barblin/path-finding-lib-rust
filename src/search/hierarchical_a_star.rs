use crate::{graph::Graph, path::PathFinding};
use crate::grid::{Direction, Grid};
use crate::node::Node;

pub struct HierarchicalAStar {
    pub heuristic: Box<dyn Fn(usize, usize, &Graph) -> u32>,
}

impl PathFinding for HierarchicalAStar {
    fn graph(&self, _source: Node, _target: Node, _graph: &Graph) -> Graph {
        return Graph::from(Vec::new());
    }

    fn grid(&self, _source: (usize, usize), _target: (usize, usize), _grid: &Grid, _directions: &[Direction]) -> Graph {
        return Graph::from(Vec::new());
    }
}