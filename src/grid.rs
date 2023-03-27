pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub costs: Vec<Vec<u32>>,
}

impl Grid {
    pub fn from(grid: &mut [&mut [u32]]) -> Grid {
        let (width, height) = (grid.len(), grid[0].len());
        let mut costs = vec![vec![0; width]; height];

        for (row, row_value) in grid.iter_mut().enumerate() {
            for (col, col_value) in row_value.iter_mut().enumerate() {
                costs[row][col] = col_value.clone();
            }
        }

        return Grid {
            width,
            height,
            costs,
        };
    }
}