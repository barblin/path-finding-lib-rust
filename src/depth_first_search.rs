use std::collections::LinkedList;

use crate::graph::{Graph, Node};
use crate::path::{PathFinding, Waypoint};

pub struct DepthFirstSearch {}

impl PathFinding for DepthFirstSearch {
    fn execute(&self, source: usize, target: usize, graph: &Graph) -> Graph {
        let start_opt = graph.nodes_lookup.get(&source);

        let result: Option<Graph> = start_opt.map(|start_node|
            probe(start_node.clone(), target, graph));

        return match result {
            Some(graph) => graph as Graph,
            None => Graph::from(Vec::new())
        };
    }
}

fn probe(start: Node, target: usize, graph: &Graph) -> Graph {
    let mut stack = LinkedList::from([Waypoint::from(None, start.edges.clone(), start, None)]);
    let mut visited: Vec<usize> = Vec::new();

    while !stack.is_empty() {
        let current = stack.pop_front().unwrap();
        let edges = current.edges.clone();
        visited.push(current.node.id);

        for edge in edges {
            let destination = edge.destination;
            let destination_node = graph.nodes_lookup.get(&destination).unwrap();
            let edges = destination_node.edges.clone();
            let previous = current.clone();

            let waypoint = Waypoint::from(Some(edge.clone()), edges, destination_node.clone(), Some(Box::new(previous)));

            if graph.nodes_lookup.get(&destination).is_some() && !visited.contains(&destination) {
                stack.push_back(waypoint)
            }

            if destination == target {
                return walk_back(stack.pop_back().unwrap());
            }
        }
    }

    return Graph::from(Vec::new());
}

fn walk_back(waypoint: Waypoint) -> Graph {
    let mut edges = Vec::new();
    let mut path = Some(Box::new(waypoint));

    while path.is_some() {
        let current = path.clone().unwrap();
        let leg = current.leg;
        let previous = current.previous;
        path = previous.clone();
        if leg.is_some() {
            edges.push(leg.unwrap());
        } else {
            Graph::from(Vec::new());
        }
    }

    return Graph::from(edges);
}