#[cfg(test)]
use std::collections::HashMap;

use crate::{graph::Graph, path::PathFinding};
#[cfg(test)]
use crate::graph::Edge;
use crate::grid::{Direction, Grid};
use crate::node::{Node, Vec3};
use crate::search::{dijkstra, dijkstra_grid};

pub fn euclidean_distance(src: &Vec3, dest: &Vec3) -> f32 {
    return src.euclidean_dist(dest);
}

pub fn manhattan_distance(src: &Vec3, dest: &Vec3) -> f32 {
    return src.manhattan_dist(dest);
}

pub struct AStar {
    pub heuristic: Box<dyn Fn(&Vec3, &Vec3) -> f32>,
}

impl PathFinding for AStar {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        graph.verify_positions();
        return dijkstra(source, target, graph, &self.heuristic);
    }

    fn grid(&self, source: (usize, usize), target: (usize, usize), grid: &Grid, directions: &[Direction]) -> Graph {
        return dijkstra_grid(source, target, grid, directions, &self.heuristic);
    }
}


#[test]
#[should_panic(expected = "You must offer node positions to the graph before using this heuristic.")]
fn missing_node_positions_should_cause_panic() {
    Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0)
    ])).get_position(&1);
}

#[test]
#[should_panic(expected = "Node position missing for given node id: 1")]
fn missing_node_position_should_cause_panic() {
    let mut graph = Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0)
    ]));

    graph.offer_positions(HashMap::from([(0, Vec3::from(0.0, 0.0, 0.0))]));

    graph.get_position(&1);
}

#[test]
fn node_position_should_be_returned() {
    let mut graph = Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0)
    ]));

    graph.offer_positions(HashMap::from([(1, Vec3::from(0.1, 0.2, 0.3))]));
    let pos = graph.get_position(&1);

    assert_eq!(0.1, pos.x);
    assert_eq!(0.2, pos.y);
    assert_eq!(0.3, pos.z);
}

#[test]
fn euclidean_heuristic_should_return_dist() {
    let mut graph = Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0)
    ]));

    graph.offer_positions(HashMap::from([
        (0, Vec3::from(20.0, 30.0, 90.0)),
        (1, Vec3::from(80.0, 44.0, 40.0))
    ]));

    assert_eq!(79.347336, euclidean_distance(graph.get_position(&0), graph.get_position(&1)));
}

#[test]
fn manhattan_heuristic_should_return_dist() {
    let mut graph = Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0)
    ]));

    graph.offer_positions(HashMap::from([
        (0, Vec3::from(2.0, 9.0, 0.0)),
        (1, Vec3::from(3.0, 5.0, 0.0))
    ]));

    assert_eq!(5.0, manhattan_distance(graph.get_position(&0), graph.get_position(&1)));
}
