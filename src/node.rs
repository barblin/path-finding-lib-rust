use crate::graph::Edge;

#[derive(Clone)]
pub struct Node {
    pub id: usize,
    pub edges: Vec<Edge>,
}

impl Node {
    pub fn from(id: usize, edges: Vec<Edge>) -> Node {
        return Node {
            id,
            edges,
        };
    }
}

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn zeroed() -> Position {
        return Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    pub fn from(x: f32, y: f32, z: f32) -> Position {
        return Position {
            x,
            y,
            z,
        };
    }
}