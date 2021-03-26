use std::thread::current;

use rand::Rng;

use crate::cell::Cell;
use crate::grid::Grid;
use crate::row::Row;

use crate::automaton::Automaton;
use crate::renderer::Renderer;

pub struct BriansBrain {
    grid: Grid,
}

impl BriansBrain {
    pub fn new(n_rows: usize, n_cols: usize) -> BriansBrain {
        let mut rows = vec![
            Row {
                cells: vec![Cell { value: 0 }; n_cols]
            };
            n_rows
        ];
        // Blinker
        // rows[4].cells[4].value = 1;
        // rows[4].cells[5].value = 1;
        // rows[4].cells[6].value = 1;
        // Glider
        // rows[4].cells[4].value = 2;
        // rows[4].cells[5].value = 2;
        // rows[4].cells[6].value = 2;
        // rows[3].cells[6].value = 2;
        // rows[2].cells[5].value = 2;
        // Random
        // let mut rng = rand::thread_rng();
        // for row in rows.iter_mut() {
        //     for col in row.cells.iter_mut() {
        //         let b: bool = rng.gen();
        //         if b {
        //             col.value = 2;
        //         }
        //     }
        // }
        rows[n_rows / 2].cells[n_cols / 2 - 1].value = 2;
        rows[n_rows / 2].cells[n_cols / 2].value = 2;
        // rows[n_rows / 2].cells[n_cols / 2 + 1].value = 2;
        rows[n_rows / 2 - 1].cells[n_cols / 2 - 1].value = 2;
        rows[n_rows / 2 - 1].cells[n_cols / 2].value = 2;
        // rows[n_rows / 2 - 1].cells[n_cols / 2 + 1].value = 2;
        // rows[n_rows / 2 - 2].cells[n_cols / 2 - 1].value = 2;
        // rows[n_rows / 2 - 2].cells[n_cols / 2].value = 2;
        // rows[n_rows / 2 - 2].cells[n_cols / 2 + 1].value = 2;
        // rows[n_rows / 2 - 3].cells[n_cols / 2 - 1].value = 2;
        // rows[n_rows / 2 - 3].cells[n_cols / 2].value = 2;
        // rows[n_rows / 2 - 3].cells[n_cols / 2 + 1].value = 2;
        BriansBrain {
            grid: Grid { rows },
        }
    }

    fn next_iteration(grid: &Grid) -> Grid {
        let mut new_rows = Vec::new();
        for row in grid.rows.iter().enumerate() {
            let mut new_row = Row { cells: Vec::new() };
            for col in row.1.cells.iter().enumerate() {
                // A cell turns on if it was off and has exactly two neighbors
                // All other on cells become dying.
                // All dying cells become off.
                let mut living_neighbours = 0;
                for dir in 0..=7 {
                    let (x, y) = grid.neib(col.0, row.0, dir).unwrap();
                    if grid.rows[y].cells[x].value == 2 {
                        living_neighbours += 1;
                    }
                }
                let current_value = col.1.value;
                match living_neighbours {
                    2 => {
                        match current_value {
                            0 => new_row.cells.push(Cell { value: 2 }),
                            1 => new_row.cells.push(Cell { value: 0 }),
                            2 => new_row.cells.push(Cell { value: 1 }),
                            _ => (),
                        }
                    },
                    _ => {
                        match current_value {
                            0 => new_row.cells.push(Cell { value: 0 }),
                            1 => new_row.cells.push(Cell { value: 0 }),
                            2 => new_row.cells.push(Cell { value: 1 }),
                            _ => (),
                        }
                    }
                }
            }
            new_rows.push(new_row);
        }
        Grid { rows: new_rows }
    }
}

impl Automaton for BriansBrain {
    fn next(&mut self) {
        self.grid = BriansBrain::next_iteration(&self.grid);
    }
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render_grid(&self.grid);
    }
}
