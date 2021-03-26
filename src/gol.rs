use rand::Rng;

use crate::cell::Cell;
use crate::grid::Grid;
use crate::row::Row;

use crate::automaton::Automaton;
use crate::renderer::Renderer;

pub struct Gol {
    grid: Grid,
}

impl Gol {
    pub fn new(n_rows: usize, n_cols: usize) -> Gol {
        let mut rows = vec![
            Row {
                cells: vec![Cell { value: 0 }; n_cols]
            };
            n_rows
        ];
        // Blinker
        // rows[4].cells[4].value = true;
        // rows[4].cells[5].value = true;
        // rows[4].cells[6].value = true;
        // Glider
        // rows[4].cells[4].value = true;
        // rows[4].cells[5].value = true;
        // rows[4].cells[6].value = true;
        // rows[3].cells[6].value = true;
        // rows[2].cells[5].value = true;
        // Random
        let mut rng = rand::thread_rng();
        for row in rows.iter_mut() {
            for col in row.cells.iter_mut() {
                let bool: bool = rng.gen();
                if bool {
                    col.value = 1;
                } else {
                    col.value = 0;
                }
            }
        }
        Gol {
            grid: Grid { rows },
        }
    }

    fn next_iteration(grid: &Grid) -> Grid {
        let mut new_rows = Vec::new();
        for row in grid.rows.iter().enumerate() {
            let mut new_row = Row { cells: Vec::new() };
            for col in row.1.cells.iter().enumerate() {
                // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                // Any live cell with two or three live neighbours lives on to the next generation.
                // Any live cell with more than three live neighbours dies, as if by overpopulation.
                // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                let mut living_neighbours = 0;
                for dir in 0..=7 {
                    let (x, y) = grid.neib(col.0, row.0, dir).unwrap();
                    if grid.rows[y].cells[x].value != 0 {
                        living_neighbours += 1;
                    }
                }
                let is_alive = col.1.value;
                match living_neighbours {
                    0..=1 => new_row.cells.push(Cell { value: 0 }),
                    2 => new_row.cells.push(Cell { value: is_alive }),
                    3 => new_row.cells.push(Cell { value: 1 }),
                    4..=8 => new_row.cells.push(Cell { value: 0 }),
                    _ => {}
                }
            }
            new_rows.push(new_row);
        }
        Grid { rows: new_rows }
    }
}

impl Automaton for Gol {
    fn next(&mut self) {
        self.grid = Gol::next_iteration(&self.grid);
    }
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render_grid(&self.grid);
    }
}
