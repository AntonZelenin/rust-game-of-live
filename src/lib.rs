mod utils;

extern crate rand;

use rand::Rng;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    height: u32,
    width: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(height: u32, width: u32) -> Self {
        let mut rng = rand::thread_rng();
        utils::set_panic_hook();
        let cells = (0..height * width)
            .map(|i| {
                if i % rng.gen_range(1, 13) == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            height,
            width,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let mut next_gen = self.cells.clone();
        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                next_gen[idx] = self.get_next_stage(row, column);
            }
        }
        self.cells = next_gen;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn get_cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Universe {
    fn get_next_stage(&self, row: u32, column: u32) -> Cell {
        match (
            self.cells[self.get_index(row, column)],
            self.live_neighbor_count(row, column),
        ) {
            (Cell::Alive, x) if x < 2 => Cell::Dead,
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            (Cell::Alive, x) if x > 3 => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (otherwise, _) => otherwise,
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (self.width * row + column) as usize
    }

    fn get_alive_neighbours_number(&self, index: u32) -> u8 {
        let (mut neighbours_num, mut column_index, mut row_index) = (0, 0, 0);
        for row_delta in [self.height - 1, 0, 1].iter().cloned() {
            for column_delta in [self.width - 1, 0, 1].iter().cloned() {
                if row_delta == 0 && column_delta == 0 {
                    continue;
                }
                column_index = (index + column_delta) % self.width;
                row_index = (index + row_delta) % self.height;
                neighbours_num += self.cells[self.get_index(column_index, row_index)] as u8;
            }
        }
        neighbours_num
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut symbol: &str;
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                symbol = if cell == Cell::Dead { "◻" } else { "◼" };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
