use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::{Edge, Graph};
use crate::grid::{Direction, Grid};
use crate::path;
use crate::path::Waypoint;
use crate::search::cost;

pub(crate) type Callback = fn(list: &mut VecDeque<Waypoint>) -> Option<Waypoint>;

pub(crate) fn pop(stack: &mut VecDeque<Waypoint>) -> Option<Waypoint> {
    return stack.pop_back();
}

pub(crate) fn dequeue(queue: &mut VecDeque<Waypoint>) -> Option<Waypoint> {
    return queue.pop_front();
}

pub(crate) fn probe_graph(start: usize, target: usize, graph: &Graph, control_flow: Callback) -> Graph {
    let mut deque = VecDeque::from([Waypoint::from(None, start, None)]);
    let mut visited: HashSet<usize> = HashSet::new();

    while let Some(current) = control_flow(&mut deque) {
        if let Some(node) = graph.nodes_lookup.get(&current.node_id) {
            let edges = node.edges.clone();
            visited.insert(current.node_id);

            for edge in edges {
                let destination = edge.destination;

                if !visited.contains(&destination) {
                    deque.push_back(Waypoint::from(
                        Some(edge.clone()),
                        destination,
                        Some(Box::new(current.clone())),
                    ));
                }

                if destination == target {
                    let edges = deque.pop_back()
                        .map_or_else(|| Vec::new(), |w| path::walk_back(w));
                    return Graph::from(edges);
                }
            }
        }
    }

    Graph::from(Vec::new())
}

pub(crate) fn probe_grid(start_coord: (usize, usize), target_coord: (usize, usize),
                         grid: &Grid, directions: &[Direction], control_flow: Callback) -> Graph {
    let start = grid.node_id(start_coord);
    let target = grid.node_id(target_coord);

    let mut deque = VecDeque::from([Waypoint::from(None, start, None)]);
    let mut visited: HashMap<usize, Waypoint> = HashMap::new();

    while let Some(current) = control_flow(&mut deque) {
        visited.insert(current.node_id, current.clone());

        if let Some(result) = go_directions(&mut deque, current, grid, directions, &visited, target) {
            return result;
        }
    }

    Graph::from(Vec::new())
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

        if !visited.contains_key(&dest_id) && cost < cost::INFINITY {
            let edge = Edge::from(dest_id, current.node_id, dest_id, cost);
            deque.push_back(Waypoint::from(Some(edge), dest_id, Some(Box::new(current.clone()))));
        }

        if dest_id == target {
            return deque.pop_back()
                .map(path::walk_back)
                .map(Graph::from);
        }
    }

    None
}