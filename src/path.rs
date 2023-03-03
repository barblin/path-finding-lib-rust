use crate::depth_first_search::DepthFirstSearch;
use crate::graph::{Edge, Graph, Node};

#[derive(Clone)]
pub(crate) struct Waypoint {
    pub leg: Option<Edge>,
    pub edges: Vec<Edge>,
    pub previous: Option<Box<Waypoint>>,
    pub node: Node,
}

impl Waypoint {
    pub fn from(edge: Option<Edge>, edges: Vec<Edge>, node: Node, previous: Option<Box<Waypoint>>) -> Waypoint {
        return Waypoint {
            leg: edge,
            edges,
            previous,
            node,
        };
    }
}

pub trait PathFinding {
    fn execute(&self, source: usize, target: usize, graph: &Graph) -> Graph;
}

pub fn find(source: usize, target: usize, graph: &Graph, path_finding: Box<dyn PathFinding>) -> Graph {
    return path_finding.execute(source, target, graph);
}


#[test]
fn find_should_find_path_with_depth_first_search_in_undirected_graph() {
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


    let graph = Graph::from(Vec::from([edge1, edge2, edge3, edge4, edge5, edge6, edge7,
        edge8, edge9, edge10, edge11, edge12, edge13, edge14]));

    let dfs = find(0, 2, &graph, Box::from(DepthFirstSearch {}) as Box<dyn PathFinding>);

    let mut total_cost: f32 = 0.0;
    for edge in dfs.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(1.4285715, total_cost);
}

#[test]
fn find_should_find_path_with_depth_first_search_in_directed_graph() {
    let edge1 = Edge::from(0, 4, 0, 7.0);
    let edge2 = Edge::from(1, 0, 2, 12.0);
    let edge3 = Edge::from(2, 0, 3, 60.0);
    let edge4 = Edge::from(3, 2, 1, 20.0);
    let edge5 = Edge::from(4, 2, 3, 32.0);
    let edge6 = Edge::from(5, 1, 0, 10.0);

    let graph = Graph::from(Vec::from([edge1, edge2, edge3, edge4, edge5, edge6]));

    let dfs = find(4, 1, &graph, Box::from(DepthFirstSearch {}) as Box<dyn PathFinding>);

    let mut total_cost: f32 = 0.0;
    for edge in dfs.edges {
        total_cost += edge.normalized_weight;
    }

    assert_eq!(39.0, total_cost);
}
