use std::thread::current;

use rand::Rng;

use crate::cell::Cell;
use crate::grid::Grid;
use crate::row::Row;

use crate::automaton::Automaton;
use crate::renderer::Renderer;

pub struct Wireworld {
    grid: Grid,
}

impl Wireworld {
    pub fn new(n_rows: usize, n_cols: usize) -> Wireworld {
        let mut rows = vec![
            Row {
                cells: vec![Cell { value: 0 }; n_cols]
            };
            n_rows
        ];
        rows[10].cells[11].value = 3;
        rows[10].cells[12].value = 3;
        rows[10].cells[13].value = 3;
        rows[10].cells[14].value = 3;
        rows[10].cells[15].value = 3;
        rows[10].cells[16].value = 3;
        rows[10].cells[17].value = 2;
        rows[10].cells[18].value = 1;

        rows[11].cells[10].value = 3;
        rows[11].cells[19].value = 3;
        rows[11].cells[20].value = 3;
        rows[11].cells[21].value = 3;
        rows[11].cells[22].value = 3;
        rows[11].cells[23].value = 2;
        rows[11].cells[24].value = 1;
        
        rows[12].cells[11].value = 3;
        rows[12].cells[12].value = 3;
        rows[12].cells[13].value = 3;
        rows[12].cells[14].value = 1;
        rows[12].cells[15].value = 2;
        rows[12].cells[16].value = 3;
        rows[12].cells[17].value = 3;
        rows[12].cells[18].value = 3;
        rows[12].cells[25].value = 3;

        rows[13].cells[24].value = 3;
        rows[13].cells[25].value = 3;
        rows[13].cells[26].value = 3;
        rows[13].cells[27].value = 3;

        rows[14].cells[24].value = 3;
        rows[14].cells[27].value = 3;
        rows[14].cells[28].value = 3;
        rows[14].cells[29].value = 2;
        rows[14].cells[30].value = 1;
        rows[14].cells[31].value = 3;

        rows[15].cells[24].value = 3;
        rows[15].cells[25].value = 3;
        rows[15].cells[26].value = 3;
        rows[15].cells[27].value = 3;

        rows[16].cells[11].value = 2;
        rows[16].cells[12].value = 1;
        rows[16].cells[13].value = 3;
        rows[16].cells[14].value = 3;
        rows[16].cells[15].value = 3;
        rows[16].cells[16].value = 3;
        rows[16].cells[17].value = 2;
        rows[16].cells[18].value = 1;
        rows[16].cells[25].value = 3;

        rows[17].cells[10].value = 3;
        rows[17].cells[19].value = 3;
        rows[17].cells[20].value = 3;
        rows[17].cells[21].value = 3;
        rows[17].cells[22].value = 3;
        rows[17].cells[23].value = 3;
        rows[17].cells[24].value = 3;
        
        rows[18].cells[11].value = 3;
        rows[18].cells[12].value = 3;
        rows[18].cells[13].value = 3;
        rows[18].cells[14].value = 3;
        rows[18].cells[15].value = 3;
        rows[18].cells[16].value = 3;
        rows[18].cells[17].value = 3;
        rows[18].cells[18].value = 3;

        Wireworld {
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
                let mut electron_neighbour = false;
                let mut electron_neighbour_cnt = 0;
                for dir in 0..=7 {
                    let (x, y) = grid.neib(col.0, row.0, dir).unwrap();
                    if grid.rows[y].cells[x].value == 1 {
                        electron_neighbour_cnt += 1;
                    }
                }
                if electron_neighbour_cnt > 0 && electron_neighbour_cnt < 3 {
                    electron_neighbour = true;
                }
                let current_value = col.1.value;
                match current_value {
                    // Empty
                    0 => new_row.cells.push(Cell { value: 0 }),
                    // Electron head
                    1 => new_row.cells.push(Cell { value: 2 }),
                    // Eletron tail
                    2 => new_row.cells.push(Cell { value: 3 }),
                    // conductor
                    3 => {
                        if electron_neighbour {
                            new_row.cells.push(Cell { value: 1 })
                        } else {
                            new_row.cells.push(Cell { value: 3 })
                        }
                    }
                    _ => panic!("Not allowed state")
                }
            }
            new_rows.push(new_row);
        }
        Grid { rows: new_rows }
    }
}

impl Automaton for Wireworld {
    fn next(&mut self) {
        self.grid = Wireworld::next_iteration(&self.grid);
    }
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render_grid(&self.grid);
    }
}
