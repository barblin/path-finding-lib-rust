use std::collections::{HashMap, VecDeque};

use crate::graph::{Edge, Graph};
use crate::grid::{Direction, Grid};
use crate::node::Node;
use crate::path;
use crate::path::Waypoint;
use crate::search::probing::go_directions;

pub(crate) fn probe_grid(start_coord: (usize, usize), target_coord: (usize, usize),
                         grid: &Grid, dirs: &[Direction]) -> Graph {
    let start = grid.node_id(start_coord);
    let target = grid.node_id(target_coord);

    let start_queue = &mut VecDeque::from([Waypoint::from(None, start, None)]);
    let target_queue = &mut VecDeque::from([Waypoint::from(None, target, None)]);

    let mut start_vis: HashMap<usize, Waypoint> = HashMap::new();
    let mut target_vis: HashMap<usize, Waypoint> = HashMap::new();

    while !start_queue.is_empty() || !target_queue.is_empty() {
        if let Some(start_result) = process_dequeue(start_queue, grid, dirs, &mut start_vis,
                                                    &mut target_vis, target) {
            return start_result;
        }

        if let Some(end_result) = process_dequeue(target_queue, grid, dirs, &mut target_vis,
                                                  &mut start_vis, start) {
            return end_result;
        }
    }

    Graph::from(Vec::new())
}

fn process_dequeue(
    deque: &mut VecDeque<Waypoint>,
    grid: &Grid,
    directions: &[Direction],
    visited: &mut HashMap<usize, Waypoint>,
    other_visited: &mut HashMap<usize, Waypoint>,
    target: usize,
) -> Option<Graph> {
    if let Some(current) = deque.pop_front() {
        let current_id = current.node_id;
        visited.insert(current_id, current.clone());

        if current_id == target {
            return Some(Graph::from(path::walk_back(current)));
        }

        if other_visited.contains_key(&current_id) {
            let mut from_current = path::walk_back(current);
            let other_edges = other_visited
                .get(&current_id)
                .map_or_else(|| Vec::new(), |w| path::walk_back(w.clone()));
            from_current.extend(other_edges);
            return Some(Graph::from(from_current));
        }

        if let Some(result) = go_directions(deque, current, grid, directions, &visited, target) {
            return Some(result);
        }
    }

    None
}

pub(crate) fn probe_graph(start: Node, target: Node, graph: &Graph) -> Graph {
    let start_queue = &mut VecDeque::from([Waypoint::from(None, start.id, None)]);
    let target_queue = &mut VecDeque::from([Waypoint::from(None, target.id, None)]);

    let mut start_visited: HashMap<usize, Waypoint> = HashMap::new();
    let mut target_visited: HashMap<usize, Waypoint> = HashMap::new();

    while !start_queue.is_empty() || !target_queue.is_empty() {
        if let Some(result_start) = process_node(start_queue, &mut start_visited,
                                                 &mut target_visited, &target, graph) {
            return Graph::from(result_start);
        }

        if let Some(result_target) = process_node(target_queue, &mut target_visited,
                                                  &mut start_visited, &start, graph) {
            return Graph::from(result_target);
        }
    }

    Graph::from(Vec::new())
}

fn process_node(queue: &mut VecDeque<Waypoint>, visited: &mut HashMap<usize, Waypoint>,
                end_visited: &mut HashMap<usize, Waypoint>,
                end: &Node, graph: &Graph, ) -> Option<Vec<Edge>> {
    if let Some(current) = queue.pop_front() {
        let result = process_edges(queue, &current, end.id, graph, &visited, &end_visited);
        visited.insert(current.node_id, current);
        result
    } else {
        None
    }
}

fn process_edges(
    queue: &mut VecDeque<Waypoint>,
    current: &Waypoint,
    target: usize,
    graph: &Graph,
    visited: &HashMap<usize, Waypoint>,
    other_visited: &HashMap<usize, Waypoint>) -> Option<Vec<Edge>>
{
    let edges = graph.nodes_lookup.get(&current.node_id).unwrap().edges.clone();

    for edge in edges {
        let destination = edge.destination;

        let waypoint = Waypoint::from(Some(edge), destination,
                                      Some(Box::new(current.clone())));

        if destination == target {
            return Some(path::walk_back(waypoint));
        }

        if !visited.contains_key(&destination) {
            queue.push_back(waypoint)
        }

        if other_visited.contains_key(&destination) {
            let mut from_current = queue.pop_back().map_or_else(|| Vec::new(), |w| path::walk_back(w));
            let other_edges = other_visited.get(&destination)
                .map_or_else(|| Vec::new(), |w| path::walk_back(w.clone()));
            from_current.extend(other_edges);
            return Some(from_current);
        }
    }

    None
}