use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::{Edge, Graph};
use crate::grid::{Direction, Grid};
use crate::path;
use crate::path::Waypoint;

pub(crate) type Callback = fn(list: &mut VecDeque<Waypoint>) -> Option<Waypoint>;

pub(crate) fn pop(stack: &mut VecDeque<Waypoint>) -> Option<Waypoint> {
    return stack.pop_back();
}

pub(crate) fn dequeue(queue: &mut VecDeque<Waypoint>) -> Option<Waypoint> {
    return queue.pop_front();
}

pub(crate) fn probe_graph(start: usize, target: usize, graph: &Graph, control_flow: Callback) -> Graph {
    let deque = &mut VecDeque::from([Waypoint::from(None, start, None)]);
    let mut visited: HashSet<usize> = HashSet::new();

    while !deque.is_empty() {
        let current = control_flow(deque).unwrap();
        let edges = graph.nodes_lookup.get(&current.node_id).unwrap().edges.clone();
        visited.insert(current.node_id);

        for edge in edges {
            let destination = edge.destination;

            if !visited.contains(&destination) {
                deque.push_back(Waypoint::from(Some(edge.clone()), destination,
                                               Some(Box::new(current.clone()))))
            }

            if destination == target {
                return Graph::from(path::walk_back(deque.pop_back()));
            }
        }
    }

    return Graph::from(Vec::new());
}

pub(crate) fn probe_grid(start_coord: (usize, usize), target_coord: (usize, usize),
                         grid: &Grid, directions: &[Direction], control_flow: Callback) -> Graph {
    let start = grid.node_id(start_coord);
    let target = grid.node_id(target_coord);

    let deque = &mut VecDeque::from([Waypoint::from(None, start, None)]);
    let mut visited: HashMap<usize, Waypoint> = HashMap::new();

    while !deque.is_empty() {
        let current = control_flow(deque).unwrap();
        visited.insert(current.node_id, current.clone());

        let result = go_directions(deque, current, grid, directions, &visited, target);

        if result.is_some() {
            return result.unwrap();
        }
    }

    return Graph::from(Vec::new());
}

pub(crate) fn go_directions(
    deque: &mut VecDeque<Waypoint>,
    current: Waypoint,
    grid: &Grid,
    directions: &[Direction],
    visited: &HashMap<usize, Waypoint>,
    target: usize,
) -> Option<Graph> {
    for direction in directions {
        let dest_coord = direction.attempt_move(grid.coords(current.node_id));

        if grid.outside(dest_coord) {
            continue;
        }

        let dest_id = grid.node_id(dest_coord);
        let cost = grid.cost(dest_id);

        if !visited.contains_key(&dest_id) && cost < f32::MAX {
            let edge = Some(Edge::from(dest_id, current.node_id, dest_id, cost));
            deque.push_back(Waypoint::from(edge, dest_id, Some(Box::new(current.clone()))));
        }

        if dest_id == target {
            return Some(Graph::from(path::walk_back(deque.pop_back())));
        }
    }

    return None;
}