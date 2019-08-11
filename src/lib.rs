mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

pub struct Universe {
    width: u32,
    length: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (self.width * row + column) as usize
    }

    pub fn get_next_stage(&self, index: u32, current_stage: u8) -> Cell {
        match self.get_alive_neighbours(index as i32) {
            v if current_stage == 0 && v == 3 => Cell::Alive,
            0 | 1 => Cell::Dead,
            2 | 3 => Cell::Alive,
            v if v > 3 => Cell::Dead,
            _ => panic!("You shouldn't use panic for such cases")
        }
    }

    fn get_alive_neighbours(&self, index: i32) -> u8 {
        let mut neighbours_num: u8 = 0;
        let mut neighbour_index: i32;
        let universe_length = (self.width * self.length) as i32;
        for row_delta in -1..1_i32 {
            for column_delta in -1..1_i32 {
                if row_delta == 0 && column_delta == 0 {
                    continue;
                }
                neighbour_index = index + (self.width as i32 * row_delta) + column_delta;
                if neighbour_index - universe_length >= 0 {
                    neighbour_index = neighbour_index - universe_length;
                }
                if let Some(Cell::Alive) = self.cells.get(neighbour_index as usize) {
                    neighbours_num += 1;
                }
            }
        }
        neighbours_num
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {

}
