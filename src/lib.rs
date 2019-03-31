mod utils;
extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
extern crate web_sys;
use web_sys::console;

use wasm_bindgen::prelude::*;

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
    cells: FixedBitSet,
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

                    // logging that records the row and column of each cell
                    // that transitioned states from live to dead or vice versa.
                    /*
                       if initial_state != next_cell {
                       log!("trans cell: row: {}, col: {}, now {:?}", row, col, next_cell);
                       }
                       */

                    // log!("    it becomes {:?}", next_cell);
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

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;
        let size = (width * height) as usize;

        // default, random, space_ship
        let cells = create_cells("random", size, width as usize);

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

        let size = (self.height * self.width) as usize;
        for i in 0..size {
            self.cells.set(i, false);
        }
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.height * self.width) as usize;
        for i in 0..size {
            self.cells.set(i, false);
        }
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        //self.cells[idx].toggle();
        if self.cells[idx] == false {
            self.cells.set(idx, true);
        }
        else if self.cells[idx] == true {
            self.cells.set(idx, false);
        }
    }

    pub fn reset(&mut self) {
        let size = (self.width * self.height) as usize;

        // default, random, space_ship
        self.cells = create_cells("random", size, self.width as usize);
    }

    pub fn clear(&mut self) {
        let size = (self.height * self.width) as usize;
        for i in 0..size {
            self.cells.set(i, false);
        }
    }

    pub fn glider(&mut self) {

    }

    pub fn pulsar(&mut self) {

    }
}

/*
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
*/

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
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
