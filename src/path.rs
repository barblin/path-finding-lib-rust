#[cfg(test)]
use std::collections::HashMap;
use std::collections::HashSet;

use crate::{graph::{Edge, Graph}};
use crate::grid::Grid;
use crate::node::Node;
#[cfg(test)]
use crate::search::AStar;
#[cfg(test)]
use crate::search::breadth_first::{BiBreadthFirstSearch, BreadthFirstSearch};
#[cfg(test)]
use crate::search::depth_first::DepthFirstSearch;
#[cfg(test)]
use crate::search::dijkstra::Dijkstra;

#[derive(Clone)]
pub(crate) struct Waypoint {
    pub leg: Option<Edge>,
    pub previous: Option<Box<Waypoint>>,
    pub node_id: usize,
}

impl Waypoint {
    pub fn from(edge: Option<Edge>, node_id: usize, previous: Option<Box<Waypoint>>) -> Waypoint {
        return Waypoint {
            leg: edge,
            previous,
            node_id,
        };
    }
}

pub trait PathFinding {
    fn graph(&self, source: Node, target: Node, graph: &Graph) -> Graph;
    fn grid(&self, source: (usize, usize), target: (usize, usize), grid: &Grid) -> Graph;
}

pub fn in_graph(source: usize, target: usize, graph: &Graph, path_finding: Box<dyn PathFinding>) -> Graph {
    let source_node = graph.nodes_lookup.get(&source);
    let target_node = graph.nodes_lookup.get(&target);

    if source_node.is_none() || target_node.is_none() {
        return Graph::from(Vec::new());
    };

    return path_finding.graph(source_node.unwrap().clone(), target_node.unwrap().clone(), graph);
}

pub fn in_grid(source: (usize, usize), target: (usize, usize),
               grid: &Grid, path_finding: Box<dyn PathFinding>) -> Graph {
    if grid.outside(source) || grid.outside(target) {
        return Graph::from(Vec::new());
    };

    return path_finding.grid(source, target, grid);
}

pub(crate) fn walk_back(waypoint: Waypoint) -> HashSet<Edge> {
    let mut edges = HashSet::new();
    let mut path = Some(Box::new(waypoint));

    while path.is_some() {
        let current = path.unwrap();
        let leg = current.leg;
        let previous = current.previous;
        path = previous.clone();
        if leg.is_some() {
            edges.insert(leg.unwrap());
        }
    }

    return edges;
}


// Testing

#[test]
fn walk_back_with_only_one_waypoint_should_succeed() {
    let waypoint = Waypoint::from(Some(Edge::from(0, 0, 1, 1.0)), 1, None);

    let mut sum_weight = 0.0;
    for edge in walk_back(waypoint) {
        sum_weight += edge.weight;
    }

    assert_eq!(1.0, sum_weight)
}

#[test]
fn walk_back_without_leg_should_succeed() {
    let waypoint = Waypoint::from(None, 1, None);

    let edges = walk_back(waypoint);
    assert_eq!(0, edges.len());
}


#[test]
fn walk_back_with_path_should_succeed() {
    let edges = walk_back(stubbed_path());
    let mut sum_weight = 0.0;
    for edge in &edges {
        sum_weight += edge.weight;
    }

    assert_eq!(10.0, sum_weight);
    assert_eq!(10, edges.len());
}

#[test]
fn should_find_path_with_depth_first_search_in_undirected_graph() {
    let graph = undirected_graph();
    let dfs = in_graph(0, 2, &graph, Box::from(DepthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in dfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(1.4285715, total_cost);
}

#[test]
fn should_find_path_with_depth_first_search_in_directed_graph() {
    let dfs = in_graph(4, 1, &directed_graph(), Box::from(DepthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in dfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(39.0, total_cost);
}

#[test]
fn should_find_path_with_breadth_first_search_in_undirected_graph() {
    let graph = undirected_graph();
    let bfs = in_graph(0, 2, &graph,
                       Box::from(crate::search::breadth_first::BreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(0.2857143, total_cost);
}

#[test]
fn should_find_path_with_breadth_first_search_in_directed_graph() {
    let bfs = in_graph(4, 1, &directed_graph(), Box::from(BreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(39.0, total_cost);
}

#[test]
fn should_find_path_with_bi_breadth_first_search_in_undirected_graph() {
    let graph = undirected_graph();
    let bfs = in_graph(0, 2, &graph, Box::from(BreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(0.2857143, total_cost);
}

#[test]
fn should_find_path_with_bi_breadth_first_search_in_directed_graph() {
    let bfs = in_graph(4, 1, &directed_graph(), Box::from(BiBreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(39.0, total_cost);
}

#[test]
fn should_find_path_with_one_edge() {
    let mut edges = Vec::new();
    edges.push(Edge::from(0, 0, 1, 1.0));
    let bfs = in_graph(0, 1, &Graph::from(edges), Box::from(BiBreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in &bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(1.0, total_cost);
    assert_eq!(1, bfs.edges.len());
}

#[test]
fn should_not_find_path_with_same_target_and_source() {
    let mut edges = Vec::new();
    edges.push(Edge::from(0, 0, 1, 1.0));
    let bfs = in_graph(1, 1, &Graph::from(edges), Box::from(BiBreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in &bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(0.0, total_cost);
    assert_eq!(0, bfs.edges.len());
}

#[test]
fn should_not_find_path_with_unknown_target() {
    let mut edges = Vec::new();
    edges.push(Edge::from(0, 0, 1, 1.0));
    let bfs = in_graph(0, 2, &Graph::from(edges), Box::from(BiBreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in &bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(0.0, total_cost);
    assert_eq!(0, bfs.edges.len());
}

#[test]
fn should_not_find_path_with_unknown_source() {
    let mut edges = Vec::new();
    edges.push(Edge::from(0, 0, 1, 1.0));
    let bfs = in_graph(2, 0, &Graph::from(edges), Box::from(BiBreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in &bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(0.0, total_cost);
    assert_eq!(0, bfs.edges.len());
}

#[test]
fn should_find_path_with_source_and_target_reversed() {
    let mut edges = Vec::new();
    edges.push(Edge::from(0, 0, 1, 1.0));
    let bfs = in_graph(1, 0, &Graph::from(edges), Box::from(BiBreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in &bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(1.0, total_cost);
    assert_eq!(1, bfs.edges.len());
}

#[cfg(test)]
fn a_star_edges() -> Vec<Edge> {
    return Vec::from([
        Edge::from(0, 0, 1, 1.0),
        Edge::from(1, 0, 2, 1.0),
        Edge::from(2, 1, 3, 1.0),
        Edge::from(3, 2, 3, 2.0),
        Edge::from(4, 3, 4, 3.0),
    ]);
}

#[cfg(test)]
fn inconsistent(source: usize, destination: usize, _graph: &Graph) -> f32 {
    return HashMap::from([
        ((0, 4), 2.0),
        ((1, 4), 4.0),
        ((2, 4), 1.0),
        ((3, 4), 1.0),
        ((4, 4), 0.0)
    ]).get(&(source, destination)).unwrap().clone();
}

#[test]
fn should_find_path_with_a_star_and_inconsistent_heuristic() {
    let a_star = in_graph(0, 4, &Graph::from(a_star_edges()),
                          Box::from(AStar { heuristic: Box::from(inconsistent) }));

    let mut total_cost: f32 = 0.0;
    for edge in &a_star.edges {
        total_cost += edge.weight;
    }

    assert_eq!(6.0, total_cost);
    assert_eq!(3, a_star.edges.len());
}

#[cfg(test)]
fn consistent(source: usize, destination: usize, _graph: &Graph) -> f32 {
    return HashMap::from([
        ((0, 4), 2.0),
        ((1, 4), 1.0),
        ((2, 4), 1.0),
        ((3, 4), 1.0),
        ((4, 4), 0.0)
    ]).get(&(source, destination)).unwrap().clone();
}

#[test]
fn should_find_path_with_a_star_and_consistent_heuristic() {
    let algo = AStar { heuristic: Box::from(consistent) };
    let a_star = in_graph(0, 4, &Graph::from(a_star_edges()),
                          Box::from(algo));

    let mut total_cost: f32 = 0.0;
    for edge in &a_star.edges {
        total_cost += edge.weight;
    }

    assert_eq!(5.0, total_cost);
    assert_eq!(3, a_star.edges.len());
}

#[test]
fn should_find_path_with_bi_breadth_first_search_in_graphs_with_one_connection() {
    let bfs = in_graph(0, 13, &graphs_with_one_connection(),
                       Box::from(BiBreadthFirstSearch {}));

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.weight;
    }

    assert_eq!(50.0, total_cost);
}

#[test]
fn should_find_path_with_dijkstra_in_graphs_with_one_connection() {
    let dijkstra = in_graph(0, 13, &graphs_with_one_connection(),
                            Box::from(Dijkstra {}));

    let mut total_cost: f32 = 0.0;
    for edge in dijkstra.edges {
        total_cost += edge.weight;
    }

    assert_eq!(50.0, total_cost);
}

#[cfg(test)]
fn undirected_graph() -> Graph {
    let edge1 = Edge::from(0, 1, 2, 0.0);
    let edge2 = Edge::from(1, 2, 1, 0.0);
    let edge3 = Edge::from(2, 2, 3, 0.1428571429);
    let edge4 = Edge::from(3, 3, 2, 0.1428571429);
    let edge5 = Edge::from(4, 1, 0, 0.2857142857);
    let edge6 = Edge::from(5, 0, 1, 0.2857142857);
    let edge7 = Edge::from(6, 3, 4, 0.2857142857);
    let edge8 = Edge::from(7, 4, 3, 0.2857142857);
    let edge9 = Edge::from(8, 1, 3, 0.4285714286);
    let edge10 = Edge::from(9, 3, 1, 0.4285714286);
    let edge11 = Edge::from(10, 0, 3, 0.8571428571);
    let edge12 = Edge::from(11, 3, 0, 0.8571428571);
    let edge13 = Edge::from(12, 0, 4, 1.0);
    let edge14 = Edge::from(13, 4, 0, 1.0);


    return Graph::from(Vec::from([edge1, edge2, edge3, edge4, edge5, edge6, edge7,
        edge8, edge9, edge10, edge11, edge12, edge13, edge14]));
}

#[cfg(test)]
fn directed_graph() -> Graph {
    let edge1 = Edge::from(0, 4, 0, 7.0);
    let edge2 = Edge::from(1, 0, 2, 12.0);
    let edge3 = Edge::from(2, 0, 3, 60.0);
    let edge4 = Edge::from(3, 2, 1, 20.0);
    let edge5 = Edge::from(4, 2, 3, 32.0);
    let edge6 = Edge::from(5, 1, 0, 10.0);

    return Graph::from(Vec::from([edge1, edge2, edge3, edge4, edge5, edge6]));
}

#[cfg(test)]
fn graphs_with_one_connection() -> Graph {
    let edge1 = Edge::from(0, 0, 4, 1.0);
    let edge2 = Edge::from(1, 1, 4, 2.0);
    let edge3 = Edge::from(2, 4, 6, 3.0);
    let edge4 = Edge::from(3, 3, 5, 4.0);
    let edge5 = Edge::from(4, 2, 5, 5.0);
    let edge6 = Edge::from(5, 5, 6, 6.0);
    let edge7 = Edge::from(6, 6, 7, 7.0);

    let edge8 = Edge::from(7, 11, 9, 8.0);
    let edge9 = Edge::from(8, 12, 9, 9.0);
    let edge10 = Edge::from(9, 9, 8, 10.0);
    let edge11 = Edge::from(10, 14, 10, 11.0);
    let edge12 = Edge::from(11, 13, 10, 12.0);
    let edge13 = Edge::from(12, 10, 8, 13.0);
    let edge14 = Edge::from(13, 8, 7, 14.0);

    let edge15 = Edge::from(14, 4, 0, 1.0);
    let edge16 = Edge::from(15, 4, 1, 2.0);
    let edge17 = Edge::from(16, 6, 4, 3.0);
    let edge18 = Edge::from(17, 5, 3, 4.0);
    let edge19 = Edge::from(18, 5, 2, 5.0);
    let edge20 = Edge::from(19, 6, 5, 6.0);
    let edge21 = Edge::from(20, 7, 6, 7.0);

    let edge22 = Edge::from(21, 9, 11, 8.0);
    let edge23 = Edge::from(22, 9, 12, 9.0);
    let edge24 = Edge::from(23, 8, 9, 10.0);
    let edge25 = Edge::from(24, 10, 14, 11.0);
    let edge26 = Edge::from(25, 10, 13, 12.0);
    let edge27 = Edge::from(26, 8, 10, 13.0);
    let edge28 = Edge::from(27, 7, 8, 14.0);

    return Graph::from(Vec::from([edge1, edge2, edge3, edge4, edge5, edge6, edge7, edge8,
        edge9, edge10, edge11, edge12, edge13, edge14, edge15, edge16, edge17, edge18, edge19, edge20,
        edge21, edge22, edge23, edge24, edge25, edge26, edge27, edge28]));
}

#[cfg(test)]
fn stubbed_path() -> Waypoint {
    let start_point = Waypoint::from(Some(Edge::from(0, 0, 1, 1.0)), 0, None);

    let mut current = start_point;
    for i in 0..10 {
        let edge_stub = Some(Edge::from(i, 0, 1, 1.0));
        current = Waypoint::from(edge_stub.clone(), 0, Some(Box::from(current.clone())))
    }

    return current.clone();
}