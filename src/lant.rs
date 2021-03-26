use crate::cell::Cell;
use crate::grid::Grid;
use crate::row::Row;

use crate::automaton::Automaton;
use crate::renderer::Renderer;

pub struct Lant {
    n_cols: usize,
    n_rows: usize,
    grid: Grid,
    current_direction: i32,
    current_position_x: usize,
    current_position_y: usize,
}

impl Lant {
    pub fn new(n_rows: usize, n_cols: usize) -> Lant {
        let rows = vec![
            Row {
                cells: vec![Cell { value: false }; n_cols]
            };
            n_rows
        ];
        Lant {
            n_cols: n_cols,
            n_rows: n_rows,
            grid: Grid { rows },
            current_direction: 0,
            current_position_x: n_cols / 2,
            current_position_y: n_rows / 2,
        }
    }

    fn next_iteration(&mut self) {
        let current_cell_value =
            &mut self.grid.rows[self.current_position_y].cells[self.current_position_x].value;
        if *current_cell_value {
            self.current_direction += 1;
        } else {
            self.current_direction -= 1;
        }
        self.current_direction = self.current_direction.rem_euclid(4);

        *current_cell_value = !(*current_cell_value);

        let mut new_pos_x = self.current_position_x as i32;
        let mut new_pos_y = self.current_position_y as i32;
        match self.current_direction {
            0 => {
                new_pos_x += 1;
            }
            1 => {
                new_pos_y += 1;
            }
            2 => {
                new_pos_x -= 1;
            }
            3 => {
                new_pos_y -= 1;
            }
            _ => panic!("Invalid direction"),
        }
        new_pos_x = new_pos_x.rem_euclid(self.n_cols as i32);
        new_pos_y = new_pos_y.rem_euclid(self.n_rows as i32);
        self.current_position_x = new_pos_x as usize;
        self.current_position_y = new_pos_y as usize;
    }
}

impl Automaton for Lant {
    fn next(&mut self) {
        self.next_iteration();
    }
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render_grid(&self.grid);
    }
}
