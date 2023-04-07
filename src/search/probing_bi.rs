use std::collections::{HashMap, VecDeque};

use crate::graph::{Edge, Graph};
use crate::grid::{Direction, Grid};
use crate::node::Node;
use crate::path;
use crate::path::Waypoint;
use crate::search::probing::{dequeue, go_directions};

pub(crate) fn probe_grid(start_coord: (usize, usize), target_coord: (usize, usize),
                         grid: &Grid, dirs: &[Direction]) -> Graph {
    let start = grid.node_id(start_coord);
    let target = grid.node_id(target_coord);

    let start_queue = &mut VecDeque::from([Waypoint::from(
        None, start, None)]);
    let target_queue = &mut VecDeque::from([Waypoint::from(
        None, target, None)]);

    let mut start_vis: HashMap<usize, Waypoint> = HashMap::new();
    let mut target_vis: HashMap<usize, Waypoint> = HashMap::new();

    while !start_queue.is_empty() || !target_queue.is_empty() {
        let start_result = process_dequeue(start_queue, grid, dirs,
                                           &mut start_vis, &mut target_vis, target);

        if start_result.is_some() {
            return start_result.unwrap();
        }

        let end_result = process_dequeue(target_queue, grid, dirs,
                                         &mut target_vis, &mut start_vis, start);

        if end_result.is_some() {
            return end_result.unwrap();
        }
    }

    return Graph::from(Vec::new());
}

fn process_dequeue(
    deque: &mut VecDeque<Waypoint>,
    grid: &Grid,
    directions: &[Direction],
    visited: &mut HashMap<usize, Waypoint>,
    other_visited: &mut HashMap<usize, Waypoint>,
    target: usize,
) -> Option<Graph> {
    if !deque.is_empty() {
        let current = dequeue(deque).unwrap();
        let current_id = (&current).node_id;
        visited.insert(current_id, current.clone());

        if current_id == target {
            return Some(Graph::from(path::walk_back(Some(current.clone()))));
        }

        if other_visited.contains_key(&current_id) {
            let mut from_current = path::walk_back(Some(current.clone()));
            let from_destination = path::walk_back(other_visited.get(&current_id).cloned());
            from_current.extend(from_destination);
            return Some(Graph::from(from_current));
        }

        let result = go_directions(deque, current.clone(), grid, directions, &visited, target);

        if result.is_some() {
            return result;
        }
    }

    return None;
}

pub(crate) fn probe_graph(start: Node, target: Node, graph: &Graph) -> Graph {
    let start_queue = &mut VecDeque::from([Waypoint::from(
        None, start.id, None)]);
    let target_queue = &mut VecDeque::from([Waypoint::from(
        None, target.id, None)]);

    let mut start_visited: HashMap<usize, Waypoint> = HashMap::new();
    let mut target_visited: HashMap<usize, Waypoint> = HashMap::new();

    while !start_queue.is_empty() || !target_queue.is_empty() {
        if !start_queue.is_empty() {
            let result_start = process_node(start_queue, &mut start_visited,
                                            &mut target_visited, &target, graph);

            if result_start.is_some() {
                return Graph::from(result_start.unwrap());
            }
        }

        if !target_queue.is_empty() {
            let result_target = process_node(target_queue, &mut target_visited,
                                             &mut start_visited, &start, graph);

            if result_target.is_some() {
                return Graph::from(result_target.unwrap());
            }
        }
    }

    return Graph::from(Vec::new());
}

fn process_node(queue: &mut VecDeque<Waypoint>,
                visited: &mut HashMap<usize, Waypoint>,
                end_visited: &mut HashMap<usize, Waypoint>,
                end: &Node,
                graph: &Graph) -> Option<Vec<Edge>> {
    let current = dequeue(queue).unwrap();

    let result = process_edges(queue, &current, end.id, graph, &visited, &end_visited);

    visited.insert(current.node_id, current.clone());

    return result;
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
            return Some(path::walk_back(Some(waypoint)));
        }

        if !visited.contains_key(&destination) {
            queue.push_back(waypoint)
        }

        if other_visited.contains_key(&destination) {
            let mut from_current = path::walk_back(queue.pop_back());
            let from_destination = path::walk_back(other_visited.get(&destination).cloned());
            from_current.extend(from_destination);
            return Some(from_current);
        }
    }

    return None;
}