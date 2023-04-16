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

pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position3D {
    pub fn zeroed() -> Position3D {
        return Position3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    pub fn from(x: f32, y: f32, z: f32) -> Position3D {
        return Position3D {
            x,
            y,
            z,
        };
    }

    pub fn euclidean_dist(&self, o: &Position3D) -> f32 {
        return ((o.x - self.x).powf(2.0) +
            (o.y - self.y).powf(2.0) +
            (o.z - self.z).powf(2.0)).sqrt();
    }

    pub fn manhattan_dist(&self, o: &Position3D) -> f32 {
        return (o.x - self.x).abs() + (o.y - self.y).abs() + (o.z - self.z).abs();
    }
}

#[test]
fn create_node_should_succeed() {
    let node = Node::from(1, vec![]);

    assert_eq!(1, node.id);
    assert!(node.edges.is_empty());
}

#[test]
fn create_zeroed_position_should_succeed() {
    let position = Position3D::zeroed();

    assert_eq!(0.0, position.x);
    assert_eq!(0.0, position.y);
    assert_eq!(0.0, position.z);
}

#[test]
fn create_position_should_succeed() {
    let position = Position3D::from(0.1, 0.2, 0.3);

    assert_eq!(0.1, position.x);
    assert_eq!(0.2, position.y);
    assert_eq!(0.3, position.z);
}

#[test]
fn test_euclidean_distance() {
    let position1 = Position3D::from(0.0, 0.0, 0.0);
    let position2 = Position3D::from(1.0, 1.0, 1.0);

    let dist = position1.euclidean_dist(&position2);
    assert_eq!(1.7320508, dist);
}

#[test]
fn test_manhattan_distance() {
    let position1 = Position3D::from(0.0, 0.0, 0.0);
    let position2 = Position3D::from(1.0, 1.0, 1.0);

    let dist = position1.manhattan_dist(&position2);
    assert_eq!(3.0, dist);
}