mod utils;
extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;

use wasm_bindgen::prelude::*;

cfg_if::cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                next.set(idx, match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two neighbors
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbors
                    // lives on to the next generations
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbors dies, as if by overpopulation
                    (true, x) if x > 3 => false,
                    // Rule 4 :: Any dead cells with exactly three live nighbors
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise
                });
            }
        }

        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
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

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let size = (width * height) as usize;

        // default, random, space_ship
        let cells = create_cells("space_ship", size, width as usize);

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_1| Cell::Dead).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        let size = (self.width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        self.height = height;
        for i in 0..self.width * height {
            cells.set(i as usize, false);
        }
        self.cells = cells;
    }
}



fn create_cells(cell_type: &str, size: usize, width: usize) -> FixedBitSet {
    match cell_type {
        "default" => return default(size),
        "space_ship" => return space_ship(size, width),
        "random" => return random(size),
        _ => panic!("Unknown cell type."),
    }
}

fn default(size: usize) -> FixedBitSet {
    let mut cells = FixedBitSet::with_capacity(size);
    for i in 0..size {
        cells.set(i, i % 2 == 0 || i % 7 == 0);
    }

    return cells;
}

fn space_ship(size: usize, width: usize) -> FixedBitSet {
    let mut cells = FixedBitSet::with_capacity(size);
    for i in 0..size {
        cells.set(i, false);
    }

    cells.set(1 + width as usize * 0, true);
    cells.set(2 + width as usize * 1, true);
    for i in 0..3 {
        cells.set(i + width as usize * 2, true);
    }

    return cells;
}

fn random(size: usize) -> FixedBitSet {
    let mut cells = FixedBitSet::with_capacity(size);
    for i in 0..size {
        if js_sys::Math::random() < 0.5 {
            cells.set(i, true);
        } else {
            cells.set(i, false);
        }
    }

    return cells;
}
