pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub costs: Vec<Vec<u32>>,
}

impl Grid {
    pub fn from(grid: &[&[u32]]) -> Grid {
        let (width, height) = (grid.len(), grid[0].len());
        let mut costs = vec![vec![0; height]; width];

        for (row, row_value) in grid.iter().enumerate() {
            for (col, col_value) in row_value.iter().enumerate() {
                costs[row][col] = col_value.clone();
            }
        }

        return Grid {
            width,
            height,
            costs,
        };
    }

    pub fn outside(&self, coord: (usize, usize)) -> bool {
        return !self.within(coord);
    }

    pub fn within(&self, coord: (usize, usize)) -> bool {
        return coord.0 < self.costs.len() && coord.1 < self.costs[coord.0].len();
    }
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