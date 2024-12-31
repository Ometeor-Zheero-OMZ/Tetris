use raylib::prelude::*;

use crate::utils::colors::get_cell_colors;

pub struct Grid {
    pub num_rows: usize,
    pub num_cols: usize,
    pub cell_size: i32,
    pub grid: Vec<Vec<i32>>,
    pub colors: Vec<Color>
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = Self {
            num_rows: 20,
            num_cols: 10,
            cell_size: 30,
            grid: vec![vec![0; 10]; 20],
            colors: get_cell_colors()
        };
        grid.init();
        grid
    }

    pub fn init(&mut self) {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                self.grid[row][col] = 0;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                let cell_value = self.grid[row][col];
                let x = col as i32 * self.cell_size + 11;
                let y = row as i32 * self.cell_size + 11;

                d.draw_rectangle(x, y, self.cell_size - 1, self.cell_size - 1, self.colors[cell_value as usize]);
            }
        }
    }

    pub fn is_cell_outside(&self, row: usize, col: usize) -> bool {
        row >= self.num_rows || col >= self.num_cols
    }

    pub fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        self.grid[row][col] == 0
    }

    pub fn clear_full_rows(&mut self) -> i32 {
        let mut completed = 0;

        for row in (0..self.num_rows).rev() {
            if self.is_row_full(row) {
                self.clear_row(row);
                completed += 1;
            } else if completed > 0 {
                self.move_row_down(row, completed);
            }
        }
        completed
    }

    pub fn is_row_full(&self, row: usize) -> bool {
        self.grid[row].iter().all(|&cell| cell != 0)
    }

    pub fn clear_row(&mut self, row: usize) {
        for col in 0..self.num_cols {
            self.grid[row][col] = 0;
        }
    }

    pub fn move_row_down(&mut self, row: usize, num_rows: i32) {
        for col in 0..self.num_cols {
            let target_row = (row as i32 + num_rows) as usize;
            self.grid[target_row][col] = self.grid[row][col];
            self.grid[row][col] = 0;
        }
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn columns(&self) -> usize {
        if !self.grid.is_empty() {
            self.grid[0].len()
        } else {
            0
        }
    }
}
