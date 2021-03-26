use rand::Rng;

use crate::cell::Cell;
use crate::grid::Grid;
use crate::row::Row;

use crate::automaton::Automaton;
use crate::renderer::Renderer;

pub struct Seeds {
    grid: Grid,
}

impl Seeds {
    pub fn new(n_rows: usize, n_cols: usize) -> Seeds {
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
        // rows[4].cells[4].value = 1;
        // rows[4].cells[5].value = 1;
        // rows[4].cells[6].value = 1;
        // rows[3].cells[6].value = 1;
        // rows[2].cells[5].value = 1;
        // Random
        // let mut rng = rand::thread_rng();
        // for row in rows.iter_mut() {
        //     for col in row.cells.iter_mut() {
        //         col.value = rng.gen();
        //     }
        // }
        rows[n_rows / 2].cells[n_cols / 2 - 1].value = 1;
        rows[n_rows / 2].cells[n_cols / 2].value = 1;
        rows[n_rows / 2].cells[n_cols / 2 + 1].value = 1;
        rows[n_rows / 2 - 1].cells[n_cols / 2 - 1].value = 1;
        rows[n_rows / 2 - 1].cells[n_cols / 2].value = 1;
        rows[n_rows / 2 - 1].cells[n_cols / 2 + 1].value = 1;
        rows[n_rows / 2 - 2].cells[n_cols / 2 - 1].value = 1;
        rows[n_rows / 2 - 2].cells[n_cols / 2].value = 1;
        rows[n_rows / 2 - 2].cells[n_cols / 2 + 1].value = 1;
        rows[n_rows / 2 - 3].cells[n_cols / 2 - 1].value = 1;
        rows[n_rows / 2 - 3].cells[n_cols / 2].value = 1;
        rows[n_rows / 2 - 3].cells[n_cols / 2 + 1].value = 1;
        Seeds {
            grid: Grid { rows },
        }
    }

    fn next_iteration(grid: &Grid) -> Grid {
        let mut new_rows = Vec::new();
        for row in grid.rows.iter().enumerate() {
            let mut new_row = Row { cells: Vec::new() };
            for col in row.1.cells.iter().enumerate() {
                // A cell turns on if it was off and has exactly two neighbors
                // All other cells turn off.
                let mut living_neighbours = 0;
                for dir in 0..=7 {
                    let (x, y) = grid.neib(col.0, row.0, dir).unwrap();
                    if grid.rows[y].cells[x].value == 1 {
                        living_neighbours += 1;
                    }
                }
                let is_alive = col.1.value;
                match living_neighbours {
                    0..=1 => new_row.cells.push(Cell { value: 0 }),
                    2 => new_row.cells.push(Cell { value: (is_alive + 1) % 2 }),
                    3..=8 => new_row.cells.push(Cell { value: 0 }),
                    _ => {}
                }
            }
            new_rows.push(new_row);
        }
        Grid { rows: new_rows }
    }
}

impl Automaton for Seeds {
    fn next(&mut self) {
        self.grid = Seeds::next_iteration(&self.grid);
    }
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render_grid(&self.grid);
    }
}
