use crate::{graph::{Graph, Node}, path::PathFinding};
use crate::search::dijkstra;

pub struct AStar {
    heuristic: dyn Fn(usize, usize, &Graph) -> f32,
}

impl PathFinding for AStar {
    fn execute(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return dijkstra(source, target, graph, &self.heuristic);
    }
}