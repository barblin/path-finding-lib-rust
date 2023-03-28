use std::collections::{HashMap, HashSet};

use ordered_float::NotNan;
use priority_queue::DoublePriorityQueue;

use crate::graph::{Edge, Graph};
use crate::node::Node;
use crate::path::PathFinding;

pub struct BreadthFirstSearch {}

pub struct Dijkstra {}

pub(crate) fn dijkstra(source: Node,
                       target: Node,
                       graph: &Graph,
                       heuristic: &dyn Fn(usize, usize, &Graph) -> f32) -> Graph {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut edges_for_node_id: HashMap<usize, Vec<Edge>> = HashMap::new();
    let mut queue: DoublePriorityQueue<usize, NotNan<f32>> = DoublePriorityQueue::new();

    queue.push(source.id, NotNan::new(0.0).unwrap());
    edges_for_node_id.insert(source.id, Vec::new());

    while !visited.contains(&target.id) && !queue.is_empty() {
        let node = queue.pop_min().unwrap();
        visited.insert(node.0);

        let edges = graph.nodes_lookup.get(&node.0).unwrap().edges.clone();
        for edge in edges {
            if !visited.contains(&edge.destination) {
                let cost = node.1 + edge.weight + heuristic(edge.destination, target.id, graph);
                queue.push(edge.destination, cost);

                let mut from_edges = edges_for_node_id.get(&node.0).unwrap().clone();
                from_edges.push(edge.clone());
                edges_for_node_id.insert(edge.destination, from_edges);
            }
        }
    }

    return match edges_for_node_id.get(&target.id) {
        None => Graph::from(Vec::new()),
        Some(edges) => Graph::from(edges.clone())
    };
}

fn dijkstra_heuristic(_source: usize, _destination: usize, _graph: &Graph) -> f32 {
    return 0.0;
}

impl PathFinding for Dijkstra {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph {
        return dijkstra(source, target, graph, &dijkstra_heuristic);
    }
}

#[test]
fn should_find_path_with_dijkstra_between_a_and_b() {
    let graph = graph();

    let dij = Dijkstra {};
    let path = dij.graph(graph.nodes_lookup.get(&0).unwrap().clone(),
                         graph.nodes_lookup.get(&1).unwrap().clone(), &graph);

    assert_eq!(3.0, calc_cost(&path.edges));
    assert_eq!(2, path.edges.len());
}

#[test]
fn should_find_path_with_dijkstra_between_a_and_c() {
    let graph = graph();

    let dij = Dijkstra {};
    let path = dij.graph(get_node(0, &graph), get_node(2, &graph), &graph);


    assert_eq!(2.0, calc_cost(&path.edges));
    assert_eq!(1, path.edges.len());
}

#[test]
fn should_find_path_with_dijkstra_between_a_and_d() {
    let graph = graph();

    let dij = Dijkstra {};
    let path = dij.graph(get_node(0, &graph), get_node(3, &graph), &graph);


    assert_eq!(5.0, calc_cost(&path.edges));
    assert_eq!(3, path.edges.len());
}

#[test]
fn should_find_path_with_dijkstra_between_a_and_e() {
    let graph = graph();

    let dij = Dijkstra {};
    let path = dij.graph(get_node(0, &graph), get_node(4, &graph), &graph);


    assert_eq!(6.0, calc_cost(&path.edges));
    assert_eq!(3, path.edges.len());
}

#[test]
fn should_find_path_with_disjoint_graphs() {
    let graph = disjoint_graph();

    let dij = Dijkstra {};
    let path = dij.graph(get_node(0, &graph), get_node(3, &graph), &graph);

    assert_eq!(0.0, calc_cost(&path.edges));
    assert_eq!(0, path.edges.len());
}

#[cfg(test)]
fn graph() -> Graph {
    return Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0),
        Edge::from(1, 0, 2, 2.0),
        Edge::from(2, 1, 2, 3.0),
        Edge::from(3, 1, 3, 2.0),
        Edge::from(4, 1, 4, 3.0),
        Edge::from(5, 2, 1, 1.0),
        Edge::from(6, 2, 3, 4.0),
        Edge::from(7, 2, 4, 5.0),
        Edge::from(8, 4, 3, 1.0)
    ]));
}

#[cfg(test)]
fn disjoint_graph() -> Graph {
    return Graph::from(Vec::from([
        Edge::from(0, 0, 1, 4.0),
        Edge::from(1, 2, 3, 2.0),
    ]));
}

#[cfg(test)]
fn get_node(id: usize, graph: &Graph) -> Node {
    return graph.nodes_lookup.get(&id).unwrap().clone();
}

#[cfg(test)]
fn calc_cost(edges: &Vec<Edge>) -> f32 {
    let mut total_cost: f32 = 0.0;
    for edge in edges {
        total_cost += edge.weight;
    }

    return total_cost;
}