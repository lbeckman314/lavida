mod utils;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;

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
    cells: Vec<Cell>,
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

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two neighbors
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbors
                    // lives on to the next generations
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbors dies, as if by overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4 :: Any dead cells with exactly three live nighbors
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
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
        let cells = create_cells("random", size, width as usize);

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}


impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn create_cells(cell_type: &str, size: usize, width: usize) -> Vec<Cell> {
    match cell_type {
        "default" => return default(size),
        "space_ship" => return space_ship(size, width),
        "random" => return random(size),
        _ => panic!("Unknown cell type."),
    }
}

fn default(size: usize) -> Vec<Cell> {
    let cells: Vec<Cell> = (0..size).map(|i| {
        if i % 2 == 0 || i % 7 == 0 {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }).collect();

    return cells;
}

fn space_ship(size: usize, width: usize) -> Vec<Cell> {
    let mut cells = Vec::with_capacity(size);
    for _i in 0..size {
        cells.push(Cell::Dead);
    }

    cells[1 + width as usize * 0] = Cell::Alive;
    cells[2 + width as usize * 1] = Cell::Alive;
    for i in 0..3 {
        cells[i + width as usize * 2] = Cell::Alive;
    }

    return cells;
}

fn random(size: usize) -> Vec<Cell> {
    let mut cells = Vec::with_capacity(size);
    for _i in 0..size {
        if js_sys::Math::random() < 0.5 {
            cells.push(Cell::Alive);
        } else {
            cells.push(Cell::Dead);
        }
    }

    return cells;
}

