use std::collections::{HashMap, LinkedList};

use crate::graph::{Edge, Graph, Node};
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

    while !deque.is_empty() {
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
                return Graph::from(path_util::walk_back(deque.pop_back().unwrap()).into_iter().collect());
            }
        }
    }

    return Graph::from(Vec::new());
}

pub(crate) fn bi_directional_probe(start: Node, target: Node, graph: &Graph) -> Graph {
    let start_queue = &mut LinkedList::from([Waypoint::from(
        None, start.edges.clone(), start.clone(), None)]);
    let target_queue = &mut LinkedList::from([Waypoint::from(
        None, target.edges.clone(), target.clone(), None)]);

    let mut start_visited: HashMap<usize, Waypoint> = HashMap::new();
    let mut target_visited: HashMap<usize, Waypoint> = HashMap::new();

    while !start_queue.is_empty() || !target_queue.is_empty() {
        let current_start = queue(start_queue);

        let result_start = process_edges(start_queue, &current_start, &target.id,
                                         graph, &start_visited, &target_visited);
        start_visited.insert(current_start.node.id, current_start.clone());

        if result_start.is_some() {
            return Graph::from(result_start.unwrap());
        }

        let current_target = queue(target_queue);
        let result_target = process_edges(target_queue, &current_target, &start.id,
                                          graph, &target_visited, &start_visited);

        target_visited.insert(current_target.node.id, current_target.clone());

        if result_target.is_some() {
            return Graph::from(result_target.unwrap());
        }
    }

    return Graph::from(Vec::new());
}

fn process_edges(
    queue: &mut LinkedList<Waypoint>,
    current: &Waypoint,
    target: &usize,
    graph: &Graph,
    visited: &HashMap<usize, Waypoint>,
    other_visited: &HashMap<usize, Waypoint>) -> Option<Vec<Edge>>
{
    let edges = current.edges.clone();

    for edge in edges {
        let destination = edge.destination;

        let waypoint = Waypoint::from(
            Some(edge), graph.nodes_lookup.get(&destination).unwrap().edges.clone(),
            graph.nodes_lookup.get(&destination).unwrap().clone(),
            Some(Box::new(current.clone())),
        );

        if destination == *target {
            return Some(path_util::walk_back(waypoint).into_iter().collect());
        }

        if !visited.contains_key(&destination) {
            queue.push_back(waypoint)
        }

        if other_visited.contains_key(&destination) {
            let mut from_current = path_util::walk_back(queue.pop_back().unwrap());
            let from_destination = path_util::walk_back(other_visited.get(&destination).unwrap().clone());
            from_current.extend(from_destination);
            return Some(from_current.into_iter().collect());
        }
    }

    return None;
}