use std::collections::LinkedList;

use crate::graph::{Graph, Node};
use crate::path::{PathFinding, Waypoint};

pub struct BreadthFirstSearch {}

pub struct BiBreadthFirstSearch {}

impl PathFinding for BreadthFirstSearch {
    fn execute(&self, source: usize, target: usize, graph: &Graph) -> Graph {
        return Graph::from(Vec::new());
    }
}

impl PathFinding for BiBreadthFirstSearch {
    fn execute(&self, source: usize, target: usize, graph: &Graph) -> Graph {
        return Graph::from(Vec::new());
    }
}