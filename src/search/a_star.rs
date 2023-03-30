#[cfg(test)]
use std::collections::HashMap;

use crate::{graph::Graph, path::PathFinding};
#[cfg(test)]
use crate::graph::Edge;
use crate::grid::{Direction, Grid};
use crate::node::{Node, Position3D};
use crate::search::dijkstra;

pub fn get_position(node_id: usize, graph: &Graph) -> &Position3D {
    match &graph.node_position_lookup {
        None => panic!("You must offer node positions to the graph before using this heuristic."),
        Some(positions) => {
            return match positions.get(&node_id) {
                None => panic!("Node position missing for given node id: {node_id}"),
                Some(position) => position
            };
        }
    };
}

pub fn euclidean_distance(source: usize, destination: usize, graph: &Graph) -> f32 {
    let src = get_position(source, graph);
    let dest = get_position(destination, graph);

    return ((dest.x - src.x).powf(2.0) + (dest.y - src.y).powf(2.0) + (dest.z - src.z).powf(2.0)).sqrt();
}

pub fn manhattan_distance(source: usize, destination: usize, graph: &Graph) -> f32 {
    let src = get_position(source, graph);
    let dest = get_position(destination, graph);

    return (dest.x - src.x).abs() + (dest.y - src.y).abs() + (dest.z - src.z).abs();
}

pub struct AStar {
    pub heuristic: Box<dyn Fn(usize, usize, &Graph) -> f32>,
}

impl PathFinding for AStar {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return dijkstra(source, target, graph, &self.heuristic);
    }

    fn grid(&self, _source: (usize, usize), _target: (usize, usize), _grid: &Grid, _directions: &[Direction]) -> Graph {
        return Graph::from(Vec::new());
    }
}


#[test]
#[should_panic(expected = "You must offer node positions to the graph before using this heuristic.")]
fn missing_node_positions_should_cause_panic() {
    get_position(1, &Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0)
    ])));
}

#[test]
#[should_panic(expected = "Node position missing for given node id: 1")]
fn missing_node_position_should_cause_panic() {
    let mut graph = Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0)
    ]));

    graph.offer_positions(HashMap::from([(0, Position3D::from(0.0, 0.0, 0.0))]));

    get_position(1, &graph);
}

#[test]
fn node_position_should_be_returned() {
    let mut graph = Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0)
    ]));

    graph.offer_positions(HashMap::from([(1, Position3D::from(0.1, 0.2, 0.3))]));
    let pos = get_position(1, &graph);

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
        (0, Position3D::from(20.0, 30.0, 90.0)),
        (1, Position3D::from(80.0, 44.0, 40.0))
    ]));

    assert_eq!(79.347336, euclidean_distance(0, 1, &graph));
}

#[test]
fn manhattan_heuristic_should_return_dist() {
    let mut graph = Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0)
    ]));

    graph.offer_positions(HashMap::from([
        (0, Position3D::from(2.0, 9.0, 0.0)),
        (1, Position3D::from(3.0, 5.0, 0.0))
    ]));

    assert_eq!(5.0, manhattan_distance(0, 1, &graph));
}
