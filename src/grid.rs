
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub costs: Vec<Vec<u32>>,
    pub size: usize,
}

impl Grid {
    pub fn from(grid: &[&[u32]]) -> Grid {
        let (width, height) = (grid.len(), grid[0].len());
        let mut costs = vec![vec![0; height]; width];
        let mut size = 0;

        for (row, row_value) in grid.iter().enumerate() {
            for (col, col_value) in row_value.iter().enumerate() {
                size += 1;
                costs[row][col] = col_value.clone();
            }
        }

        return Grid {
            width,
            height,
            costs,
            size,
        };
    }

    pub fn outside(&self, coord: (usize, usize)) -> bool {
        return !self.within(coord);
    }

    pub fn within(&self, coord: (usize, usize)) -> bool {
        return coord.0 < self.costs.len() && coord.1 < self.costs[coord.0].len();
    }

    pub fn node_id(&self, coord: (usize, usize)) -> usize {
        if self.outside(coord) {
            panic!("Coordinate is outside of matrix");
        }

        return self.costs[coord.0].len() * coord.0 + coord.1;
    }

    pub fn cost(&self, node_id: usize) -> u32 {
        if self.size <= node_id {
            panic!("Node id exceeds grid size");
        }

        return self.costs[node_id / self.height][node_id % self.width];
    }
}

#[test]
fn get_cost_with_node_id_left_upper() {
    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0],
        &[3, 4, 7]
    ]);


    assert_eq!(4, grid.cost(0));
}

#[test]
fn get_cost_with_node_id_center() {
    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0],
        &[3, 4, 7]
    ]);


    assert_eq!(1, grid.cost(4));
}


#[test]
fn get_cost_with_node_id_right_lower() {
    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0],
        &[3, 4, 7]
    ]);


    assert_eq!(7, grid.cost(8));
}

#[test]
#[should_panic(expected = "Node id exceeds grid size")]
fn get_cost_with_node_id_should_panic() {
    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0],
        &[3, 4, 7]
    ]);


    grid.cost(9);
}

#[test]
fn node_id_should_return_center_node_id() {
    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0],
        &[3, 4, 7]
    ]);


    assert_eq!(4, grid.node_id((1, 1)));
}

#[test]
fn node_id_should_return_left_upper_node_id() {
    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0],
        &[3, 4, 7]
    ]);


    assert_eq!(0, grid.node_id((0, 0)));
}

#[test]
fn node_id_should_return_right_lower_node_id() {
    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0],
        &[3, 4, 7]
    ]);


    assert_eq!(8, grid.node_id((2, 2)));
}

#[test]
#[should_panic(expected = "Coordinate is outside of matrix")]
fn node_id_should_panic() {
    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0],
        &[3, 4, 7]
    ]);


    assert_eq!(8, grid.node_id((2, 3)));
}

#[test]
fn coord_should_be_within() {
    let coord: (usize, usize) = (0, 0);

    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0]
    ]);

    assert!(grid.within(coord));
    assert!(!grid.outside(coord));
}

#[test]
fn coord_row_should_be_outside() {
    let coord: (usize, usize) = (2, 0);

    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0]
    ]);

    assert!(grid.outside(coord));
}

#[test]
fn coord_col_should_be_outside() {
    let coord: (usize, usize) = (0, 3);

    let grid = Grid::from(&[
        &[4, 2, 1],
        &[2, 1, 0]
    ]);

    assert!(grid.outside(coord));
}

#[test]
fn from_should_create_grid() {
    let grid_matrix: &[&[u32]] = &[
        &[0, 4, 0, 0, 0, 0, 0, 8, 0],
        &[4, 0, 8, 0, 0, 0, 0, 11, 0],
        &[0, 8, 0, 7, 0, 4, 0, 0, 2],
        &[0, 0, 7, 0, 9, 14, 0, 0, 0],
        &[0, 0, 0, 9, 0, 10, 0, 0, 0],
        &[0, 0, 4, 14, 10, 0, 2, 0, 0],
        &[0, 0, 0, 0, 0, 2, 0, 1, 6],
        &[8, 11, 0, 0, 0, 0, 1, 0, 7],
        &[0, 0, 2, 0, 0, 0, 6, 7, 0]
    ];

    let grid = Grid::from(grid_matrix);
    assert_eq!(9, grid.height);
    assert_eq!(9, grid.width);
    assert_eq!(7, grid.costs[8][7]);
}