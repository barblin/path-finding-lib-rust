use std::collections::LinkedList;
use crate::graph::{Graph, Node};

use crate::path::Waypoint;
use crate::path_util;

pub(crate) type Callback = fn(list: &mut LinkedList<Waypoint>) -> Waypoint;

pub(crate) fn stack(stack: &mut LinkedList<Waypoint>) -> Waypoint {
    return stack.pop_back().unwrap();
}

pub(crate) fn queue(queue: &mut LinkedList<Waypoint>) -> Waypoint {
    return queue.pop_front().unwrap();
}

pub(crate) fn probe(start: Node, target: usize, graph: &Graph, control_flow: Callback) -> Graph {
    let deque = &mut LinkedList::from([Waypoint::from(
        None, start.edges.clone(), start, None)]);
    let mut visited: Vec<usize> = Vec::new();

    while !stack.is_empty() {
        let current = (control_flow)(deque);
        let edges = current.edges.clone();
        visited.push(current.node.id);

        for edge in edges {
            let destination = edge.destination;
            let destination_node = graph.nodes_lookup.get(&destination).unwrap();
            let edges = destination_node.edges.clone();
            let previous = current.clone();

            let waypoint = Waypoint::from(
                Some(edge.clone()), edges,
                destination_node.clone(),
                Some(Box::new(previous)),
            );

            if graph.nodes_lookup.get(&destination).is_some() && !visited.contains(&destination) {
                deque.push_back(waypoint)
            }

            if destination == target {
                return path_util::walk_back(deque.pop_back().unwrap());
            }
        }
    }

    return Graph::from(Vec::new());
}
