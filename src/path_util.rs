use crate::graph::Graph;
use crate::path::Waypoint;

pub(crate) fn walk_back(waypoint: Waypoint) -> Graph {
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
