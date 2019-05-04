mod utils;
extern crate js_sys;
extern crate web_sys;
use web_sys::console;

use wasm_bindgen::prelude::*;
use std::fmt;

// A macro to provide 'println!(..)'-style syntac for 'console.log' logging.
macro_rules! log{
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

cfg_if::cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer <'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
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
        // let _timer = Timer::new("Universe::tick");

        let mut next = {
            // let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };

        // let _timer = Timer::new("new generation");
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                /*
                   log!(
                   "cell[{}, {}] is initially {:?} and has {} live neighbors",
                   row,
                   col,
                   cell,
                   live_neighbors
                   );
                   */

                // let initial_state = cell;

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

                    // logging that records the row and column of each cell
                    // that transitioned states from live to dead or vice versa.
                    /*
                       if initial_state != next_cell {
                       log!("trans cell: row: {}, col: {}, now {:?}", row, col, next_cell);
                       }
                       */

                // log!("    it becomes {:?}", next_cell);

                next[idx] = next_cell;
            }
        }

        // let _timer = Timer::new("free old cells");
        self.cells = next;
    }


    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let size = (width * height) as usize;

        // default, random, space_ship
        let cells = create_cells("random", size, width as usize);

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn mynew() -> Universe {
        utils::set_panic_hook();
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

    pub fn set_size(&mut self, width: u32, height: u32) -> Universe {
        self.width = width;
        self.height = height;
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
        self.height = height;
        self.cells = (0..self.width * height).map(|_1| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn reset(&mut self) {
        let size = (self.width * self.height) as usize;

        // default, random, space_ship
        self.cells = create_cells("random", size, self.width as usize);
    }

    pub fn clear(&mut self) {
        self.cells = (0..self.width * self.height).map(|_1| Cell::Dead).collect();
    }

    pub fn glider(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);

        self.cells[1 + column as usize + self.width as usize * (0 + row) as usize] = Cell::Alive;
        self.cells[2 + column as usize + self.width as usize * (1 + row) as usize] = Cell::Alive;
        for i in 0..3 {
            self.cells[i + column as usize + self.width as usize * (2 + row) as usize] = Cell::Alive;
        }
    }

    pub fn pulsar(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);

    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
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

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }

    fn birth(&mut self) {
        *self = Cell::Alive;
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
