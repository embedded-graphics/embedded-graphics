//! https://github.com/rustwasm/wasm_game_of_life

use heapless::consts::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: heapless::Vec<Cell, U32768>,
    next_cells: heapless::Vec<Cell, U32768>,
}

impl Universe {
    pub fn new(
        width: u32,
        height: u32,
        mut cells: heapless::Vec<Cell, U32768>,
        mut next_cells: heapless::Vec<Cell, U32768>,
    ) -> Universe {
        cells.clear();
        next_cells.clear();

        for i in 0..width * height {
            let cell = if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            };
            cells.push(cell).ok();
            next_cells.push(Cell::Dead).ok();
        }

        Universe {
            width,
            height,
            cells,
            next_cells,
        }
    }

    // min 0,0 = 0, max 127,159 = 127 * 160 + 159 = 20479
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

    pub fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                self.next_cells[idx] = next_cell;
            }
        }

        //swap the working copy
        core::mem::swap(&mut self.next_cells, &mut self.cells);
    }

    //todo lifetime ok?
    pub fn iter(&mut self) -> impl Iterator<Item = (u32, u32, Cell)> + '_ {
        let width = self.width;
        let size = (self.width * self.height) as usize;
        self.cells[..size]
            .iter()
            .enumerate()
            .map(move |(idx, cell)| {
                let index = idx as u32;
                let row = index / width;
                let column = index % width;
                (row, column, *cell)
            })
    }
}
