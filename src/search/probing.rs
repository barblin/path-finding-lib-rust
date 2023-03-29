use std::collections::{HashMap, LinkedList};

use crate::graph::{Edge, Graph};
use crate::node::Node;
use crate::path;
use crate::path::Waypoint;

pub(crate) type Callback = fn(list: &mut LinkedList<Waypoint>) -> Waypoint;

pub(crate) fn pop(stack: &mut LinkedList<Waypoint>) -> Waypoint {
    return stack.pop_back().unwrap();
}

pub(crate) fn dequeue(queue: &mut LinkedList<Waypoint>) -> Waypoint {
    return queue.pop_front().unwrap();
}

pub(crate) fn probe(start: usize, target: usize, graph: &Graph, control_flow: Callback) -> Graph {
    let deque = &mut LinkedList::from([Waypoint::from(None, start, None)]);
    let mut visited: Vec<usize> = Vec::new();

    while !deque.is_empty() {
        let current = (control_flow)(deque);
        let edges = graph.nodes_lookup.get(&current.node_id).unwrap().edges.clone();
        visited.push(current.node_id);

        for edge in edges {
            let destination = edge.destination;

            let waypoint = Waypoint::from(Some(edge.clone()), destination,
                                          Some(Box::new(current.clone())), );

            if !visited.contains(&destination) {
                deque.push_back(waypoint)
            }

            if destination == target {
                return Graph::from(path::walk_back(deque.pop_back().unwrap()).into_iter().collect());
            }
        }
    }

    return Graph::from(Vec::new());
}

pub(crate) fn bi_directional_probe(start: Node, target: Node, graph: &Graph) -> Graph {
    let start_queue = &mut LinkedList::from([Waypoint::from(
        None, start.id, None)]);
    let target_queue = &mut LinkedList::from([Waypoint::from(
        None, target.id, None)]);

    let mut start_visited: HashMap<usize, Waypoint> = HashMap::new();
    let mut target_visited: HashMap<usize, Waypoint> = HashMap::new();

    while !start_queue.is_empty() || !target_queue.is_empty() {
        let result_start = process_node(start_queue, &mut start_visited,
                                        &mut target_visited, &target, graph);

        if result_start.is_some() {
            return Graph::from(result_start.unwrap());
        }

        let result_target = process_node(target_queue, &mut target_visited,
                                         &mut start_visited, &start, graph);

        if result_target.is_some() {
            return Graph::from(result_target.unwrap());
        }
    }

    return Graph::from(Vec::new());
}

fn process_node(queue: &mut LinkedList<Waypoint>,
                visited: &mut HashMap<usize, Waypoint>,
                end_visited: &mut HashMap<usize, Waypoint>,
                end: &Node,
                graph: &Graph) -> Option<Vec<Edge>> {
    let current = dequeue(queue);

    let result = process_edges(queue, &current, end.id, graph, &visited, &end_visited);

    visited.insert(current.node_id, current.clone());

    return result;
}

fn process_edges(
    queue: &mut LinkedList<Waypoint>,
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
                                      Some(Box::new(current.clone())), );

        if destination == target {
            return Some(path::walk_back(waypoint).into_iter().collect());
        }

        if !visited.contains_key(&destination) {
            queue.push_back(waypoint)
        }

        if other_visited.contains_key(&destination) {
            let mut from_current = path::walk_back(queue.pop_back().unwrap());
            let from_destination = path::walk_back(other_visited.get(&destination).unwrap().clone());
            from_current.extend(from_destination);
            return Some(from_current.into_iter().collect());
        }
    }

    return None;
}