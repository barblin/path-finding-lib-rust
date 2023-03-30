pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn attempt_move(&self, coords: (usize, usize)) -> (usize, usize) {
        let opt_coord = match self {
            Direction::Up => (coords.0.checked_sub(1), Some(coords.1)),
            Direction::Down => (coords.0.checked_add(1), Some(coords.1)),
            Direction::Left => (Some(coords.0), coords.1.checked_sub(1)),
            Direction::Right => (Some(coords.0), coords.1.checked_add(1)),
            Direction::UpLeft => (coords.0.checked_sub(1), coords.1.checked_sub(1)),
            Direction::UpRight => (coords.0.checked_sub(1), coords.1.checked_add(1)),
            Direction::DownLeft => (coords.0.checked_add(1), coords.1.checked_sub(1)),
            Direction::DownRight => (coords.0.checked_add(1), coords.1.checked_add(1)),
        };


        if opt_coord.0.and(opt_coord.1).is_some() {
            return (opt_coord.0.unwrap(), opt_coord.1.unwrap());
        }

        return coords;
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub costs: Vec<Vec<f32>>,
    pub size: usize,
}

impl Grid {
    pub fn from(grid: &[&[f32]]) -> Grid {
        if grid.is_empty() || grid[0].is_empty() {
            panic!("Given grid should not be empty")
        }

        let (width, height) = (grid.len(), grid[0].len());
        let mut costs = vec![vec![0.0; height]; width];

        for (row, row_value) in grid.iter().enumerate() {
            for (col, col_value) in row_value.iter().enumerate() {
                costs[row][col] = col_value.clone();
            }
        }

        return Grid {
            width,
            height,
            costs,
            size: width * height,
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

    pub fn coords(&self, node_id: usize) -> (usize, usize) {
        if self.size <= node_id {
            panic!("Node id exceeds grid size");
        }

        return (node_id / self.height, node_id % self.width);
    }

    pub fn cost(&self, node_id: usize) -> f32 {
        let (row, col) = self.coords(node_id);
        return self.costs[row][col];
    }
}


// Testing
#[test]
fn subtract_below_zero_should_be_max() {
    let coord = Direction::UpLeft.attempt_move((0, 0));

    assert_eq!(0, coord.0);
    assert_eq!(0, coord.1)
}

// Testing
#[test]
fn add_above_max_should_remain() {
    let coord = Direction::DownRight.attempt_move((usize::MAX, usize::MAX));

    assert_eq!(usize::MAX, coord.0);
    assert_eq!(usize::MAX, coord.1)
}

#[test]
fn up_direction_should_move_coordinate() {
    let coord = Direction::Up.attempt_move((1, 1));

    assert_eq!(0, coord.0);
    assert_eq!(1, coord.1)
}

#[test]
fn down_direction_should_move_coordinate() {
    let coord = Direction::Down.attempt_move((1, 1));

    assert_eq!(2, coord.0);
    assert_eq!(1, coord.1)
}

#[test]
fn left_direction_should_move_coordinate() {
    let coord = Direction::Left.attempt_move((1, 1));

    assert_eq!(1, coord.0);
    assert_eq!(0, coord.1)
}

#[test]
fn right_direction_should_move_coordinate() {
    let coord = Direction::Right.attempt_move((1, 1));

    assert_eq!(1, coord.0);
    assert_eq!(2, coord.1)
}

#[test]
fn up_left_direction_should_move_coordinate() {
    let coord = Direction::UpLeft.attempt_move((1, 1));

    assert_eq!(0, coord.0);
    assert_eq!(0, coord.1)
}

#[test]
fn up_right_direction_should_move_coordinate() {
    let coord = Direction::UpRight.attempt_move((1, 1));

    assert_eq!(0, coord.0);
    assert_eq!(2, coord.1)
}

#[test]
fn down_left_direction_should_move_coordinate() {
    let coord = Direction::DownLeft.attempt_move((1, 1));

    assert_eq!(2, coord.0);
    assert_eq!(0, coord.1)
}

#[test]
fn down_right_direction_should_move_coordinate() {
    let coord = Direction::DownRight.attempt_move((1, 1));

    assert_eq!(2, coord.0);
    assert_eq!(2, coord.1)
}

#[test]
fn get_cost_with_node_id_left_upper() {
    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0],
        &[3.0, 4.0, 7.0]
    ]);


    assert_eq!(4.0, grid.cost(0));
    assert_eq!(9, grid.size);
}

#[test]
fn get_cost_with_node_id_center() {
    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0],
        &[3.0, 4.0, 7.0]
    ]);


    assert_eq!(1.0, grid.cost(4));
}


#[test]
fn get_cost_with_node_id_right_lower() {
    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0],
        &[3.0, 4.0, 7.0]
    ]);


    assert_eq!(7.0, grid.cost(8));
}

#[test]
#[should_panic(expected = "Node id exceeds grid size")]
fn get_cost_with_node_id_should_panic() {
    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0],
        &[3.0, 4.0, 7.0]
    ]);


    grid.cost(9);
}

#[test]
fn node_id_should_return_center_node_id() {
    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0],
        &[3.0, 4.0, 7.0]
    ]);


    assert_eq!(4, grid.node_id((1, 1)));
}

#[test]
fn node_id_should_return_left_upper_node_id() {
    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0],
        &[3.0, 4.0, 7.0]
    ]);


    assert_eq!(0, grid.node_id((0, 0)));
}

#[test]
fn node_id_should_return_right_lower_node_id() {
    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0],
        &[3.0, 4.0, 7.0]
    ]);


    assert_eq!(8, grid.node_id((2, 2)));
}

#[test]
#[should_panic(expected = "Coordinate is outside of matrix")]
fn node_id_should_panic() {
    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0],
        &[3.0, 4.0, 7.0]
    ]);


    assert_eq!(8, grid.node_id((2, 3)));
}

#[test]
fn coord_should_be_within() {
    let coord: (usize, usize) = (0, 0);

    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0]
    ]);

    assert!(grid.within(coord));
    assert!(!grid.outside(coord));
}

#[test]
fn coord_row_should_be_outside() {
    let coord: (usize, usize) = (2, 0);

    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0]
    ]);

    assert!(grid.outside(coord));
}

#[test]
fn coord_col_should_be_outside() {
    let coord: (usize, usize) = (0, 3);

    let grid = Grid::from(&[
        &[4.0, 2.0, 1.0],
        &[2.0, 1.0, 0.0]
    ]);

    assert!(grid.outside(coord));
}

#[test]
fn from_should_create_grid() {
    let grid_matrix: &[&[f32]] = &[
        &[0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0],
        &[4.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 11.0, 0.0],
        &[0.0, 8.0, 0.0, 7.0, 0.0, 4.0, 0.0, 0.0, 2.0],
        &[0.0, 0.0, 7.0, 0.0, 9.0, 14.0, 0.0, 0.0, 0.0],
        &[0.0, 0.0, 0.0, 9.0, 0.0, 10.0, 0.0, 0.0, 0.0],
        &[0.0, 0.0, 4.0, 14.0, 10.0, 0.0, 2.0, 0.0, 0.0],
        &[0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 1.0, 6.0],
        &[8.0, 11.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 7.0],
        &[0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 6.0, 7.0, 0.0]
    ];

    let grid = Grid::from(grid_matrix);
    assert_eq!(9, grid.height);
    assert_eq!(9, grid.width);
    assert_eq!(7.0, grid.costs[8][7]);
    assert_eq!(81, grid.size);
}

#[test]
#[should_panic(expected = "Given grid should not be empty")]
fn from_with_no_rows_should_panic() {
    Grid::from(&[]);
}

#[test]
#[should_panic(expected = "Given grid should not be empty")]
fn from_with_no_columns_should_panic() {
    Grid::from(&[&[]]);
}
