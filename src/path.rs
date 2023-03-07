use std::collections::HashSet;
use crate::graph::{Edge, Graph, Node};

#[derive(Clone)]
pub(crate) struct Waypoint {
    pub leg: Option<Edge>,
    pub previous: Option<Box<Waypoint>>,
    pub node: Node,
}

impl Waypoint {
    pub fn from(edge: Option<Edge>, node: Node, previous: Option<Box<Waypoint>>) -> Waypoint {
        return Waypoint {
            leg: edge,
            previous,
            node,
        };
    }
}

pub trait PathFinding {
    fn execute(&self, source: Node, target: Node, graph: &Graph) -> Graph;
}

pub fn find(source: usize, target: usize, graph: &Graph, path_finding: Box<dyn PathFinding>) -> Graph {
    let source_node = graph.nodes_lookup.get(&source);
    let target_node = graph.nodes_lookup.get(&target);

    if source_node.is_none() || target_node.is_none() {
        return Graph::from(Vec::new());
    };

    return path_finding.execute(source_node.unwrap().clone(), target_node.unwrap().clone(), graph);
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

#[test]
fn walk_back_with_only_one_waypoint_should_succeed() {
    let waypoint = Waypoint::from(Some(Edge::from(0, 0, 1, 1.0)), Node {
        id: 1,
        edges: Vec::new(),
    }, None);

    let mut sum_weight = 0.0;
    for edge in walk_back(waypoint) {
        sum_weight += edge.normalized_weight;
    }

    assert_eq!(1.0, sum_weight)
}

#[test]
fn walk_back_without_leg_should_succeed() {
    let waypoint = Waypoint::from(None, Node {
        id: 1,
        edges: Vec::new(),
    }, None);

    let edges = walk_back(waypoint);
    assert_eq!(0, edges.len());
}


#[test]
fn walk_back_with_path_should_succeed() {
    let edges = walk_back(stubbed_path());
    let mut sum_weight = 0.0;
    for edge in &edges {
        sum_weight += edge.normalized_weight;
    }

    assert_eq!(10.0, sum_weight);
    assert_eq!(10, edges.len());
}

#[test]
fn should_find_path_with_depth_first_search_in_undirected_graph() {
    let graph = undirected_graph();
    let dfs = find(0, 2, &graph,
                   Box::from(crate::depth_first::DepthFirstSearch {}) as Box<dyn PathFinding>);

    let mut total_cost: f32 = 0.0;
    for edge in dfs.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(1.4285715, total_cost);
}

#[test]
fn should_find_path_with_depth_first_search_in_directed_graph() {
    let dfs = find(4, 1, &directed_graph(),
                   Box::from(crate::depth_first::DepthFirstSearch {}) as Box<dyn PathFinding>);

    let mut total_cost: f32 = 0.0;
    for edge in dfs.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(39.0, total_cost);
}

#[test]
fn should_find_path_with_breadth_first_search_in_undirected_graph() {
    let graph = undirected_graph();
    let bfs = find(0, 2, &graph,
                   Box::from(crate::breadth_first::BreadthFirstSearch {}) as Box<dyn PathFinding>);

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(0.2857143, total_cost);
}

#[test]
fn should_find_path_with_breadth_first_search_in_directed_graph() {
    let bfs = find(4, 1, &directed_graph(),
                   Box::from(crate::breadth_first::BreadthFirstSearch {}) as Box<dyn PathFinding>);

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(39.0, total_cost);
}

#[test]
fn should_find_path_with_bi_breadth_first_search_in_undirected_graph() {
    let graph = undirected_graph();
    let bfs = find(0, 2, &graph,
                   Box::from(crate::breadth_first::BiBreadthFirstSearch {}) as Box<dyn PathFinding>);

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(0.2857143, total_cost);
}

#[test]
fn should_find_path_with_bi_breadth_first_search_in_directed_graph() {
    let bfs = find(4, 1, &directed_graph(),
                   Box::from(crate::breadth_first::BiBreadthFirstSearch {}) as Box<dyn PathFinding>);

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(37.0, total_cost);
}

#[test]
fn should_find_path_with_bi_breadth_first_search_in_graphs_with_one_connection() {
    let bfs = find(0, 13, &graphs_with_one_connection(),
                   Box::from(crate::breadth_first::BiBreadthFirstSearch {}) as Box<dyn PathFinding>);

    let mut total_cost: f32 = 0.0;
    for edge in bfs.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(50.0, total_cost);
}

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

fn directed_graph() -> Graph {
    let edge1 = Edge::from(0, 4, 0, 7.0);
    let edge2 = Edge::from(1, 0, 2, 12.0);
    let edge3 = Edge::from(2, 0, 3, 60.0);
    let edge4 = Edge::from(3, 2, 1, 20.0);
    let edge5 = Edge::from(4, 2, 3, 32.0);
    let edge6 = Edge::from(5, 1, 0, 10.0);

    return Graph::from(Vec::from([edge1, edge2, edge3, edge4, edge5, edge6]));
}

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

    return Graph::from(Vec::from([edge1, edge2, edge3, edge4, edge5, edge6, edge7, edge8,
        edge9, edge10, edge11, edge12, edge13, edge14]));
}

fn stubbed_path() -> Waypoint {
    let start_point = Waypoint::from(Some(Edge::from(0, 0, 1, 1.0)), Node {
        id: 0,
        edges: Vec::new(),
    }, None);

    let mut current = start_point;
    for i in 0..10 {
        let edge_stub = Some(Edge::from(i, 0, 1, 1.0));
        current = Waypoint::from(edge_stub.clone(), Node {
            id: 0,
            edges: Vec::new(),
        }, Some(Box::from(current.clone())))
    }

    return current.clone();
}