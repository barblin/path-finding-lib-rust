use std::collections::HashSet;
use crate::graph::Edge;
use crate::path::Waypoint;

pub(crate) fn walk_back(waypoint: Waypoint) -> HashSet<Edge> {
    let mut edges = HashSet::new();
    let mut path = Some(Box::new(waypoint));

    while path.is_some() {
        let current = path.clone().unwrap();
        let leg = current.leg;
        let previous = current.previous;
        path = previous.clone();
        if leg.is_some() {
            edges.insert(leg.unwrap());
        }
    }

    return edges;
}
