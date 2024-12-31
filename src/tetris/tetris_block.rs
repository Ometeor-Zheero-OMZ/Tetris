use std::time::SystemTime;

use rand::RngCore;
use raylib::prelude::*;

use crate::{rand::SplitMixXoshiro256Rng, utils::colors::get_cell_colors};

use super::{blocks::*, position::Position};

#[derive(Clone)]
pub struct TetrisBlock {
    pub id: usize,
    pub cell_size: i32,
    pub rotation_state: usize,
    pub colors: Vec<Color>,
    pub row_offset: i32,
    pub column_offset: i32,
    pub cells: Vec<Vec<Position>>
}

impl TetrisBlock {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Generate seed with UNIX_EPOCH")
            .as_nanos() as u64;
        let mut rng = SplitMixXoshiro256Rng::new(seed);

        let cells = get_block_position(rng.next_u32());

        Self {
            id: 0,
            cell_size: 30,
            rotation_state: 0,
            colors: get_cell_colors(),
            row_offset: 0,
            column_offset: 0,
            cells
        }
    }

    pub fn draw(&self, offset_x: i32, offset_y: i32, d: &mut RaylibDrawHandle) {
        let tiles = self.get_cell_positions();
        for tile in tiles {
            d.draw_rectangle(
                tile.column * self.cell_size + offset_x,
                tile.row * self.cell_size + offset_y,
                self.cell_size - 1,
                self.cell_size - 1,
                self.colors[self.id as usize]
            );
        }
    }

    pub fn move_block(&mut self, rows: i32, columns: i32) {
        self.row_offset += rows;
        self.column_offset += columns;
    }

    pub fn get_cell_positions(&self) -> Vec<Position> {
        let tiles = &self.cells[self.rotation_state];
        let mut moved_tiles = Vec::new();

        for tile in tiles {
            let new_pos = Position {
                row: tile.row + self.row_offset,
                column: tile.column + self.column_offset
            };
            moved_tiles.push(new_pos);
        }
        moved_tiles
    }

    pub fn rotate(&mut self) {
        self.rotation_state += 1;
        if self.rotation_state == self.cells.len() {
            self.rotation_state = 0;
        }
    }

    pub fn undo_rotation(&mut self) {
        if self.rotation_state == 0 {
            self.rotation_state = self.cells.len() - 1;
        } else {
            self.rotation_state -= 1;
        }
    }
}