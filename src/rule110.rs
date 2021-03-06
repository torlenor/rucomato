use rand::Rng;

use crate::row::Row;
use crate::cell::Cell;

use crate::renderer::Renderer;
use crate::automaton::Automaton;

pub struct Rule110 {
    current_row: Row,
}

impl Rule110 {
    pub fn new(cols: usize) -> Rule110 {
        let mut rng = rand::thread_rng();
        let mut row = Row { cells: Vec::new() };
        for _ in 1..=cols {
            let b: bool = rng.gen();
            if b {
                row.cells.push(Cell { value: 1 });
            } else {
                row.cells.push(Cell { value: 0 });
            }
        }
        Rule110 { current_row: row}
    }
    fn determine_new_value(byte: &u8) -> Result<i32, &'static str> {
        match byte {
            0b111 => Ok(0),
            0b110 => Ok(1),
            0b101 => Ok(1),
            0b100 => Ok(0),
            0b011 => Ok(1),
            0b010 => Ok(1),
            0b001 => Ok(1),
            0b000 => Ok(0),
            _ => Err("Unhandled case"),
        }
    }

    fn apply_rule110(row: &Row) -> Row {
        let cols = row.len();
        let mut new_row = Row { cells: Vec::new() };
        for c in 0..(cols) {
            let l_neib = ((c as i32)-1).rem_euclid(cols as i32) as usize;
            let r_neib = (c+1) % cols;
            let mut byte: u8 = 0b0000_0000;
            if row.cells[l_neib].value != 0 {
                byte |= 0b0000_0100;
            }
            if row.cells[c].value != 0 {
                byte |= 0b0000_0010;
            }
            if row.cells[r_neib].value != 0 {
                byte |= 0b0000_0001;
            }
            let new_cell_value = Rule110::determine_new_value(&byte);
            match new_cell_value {
                Ok(v) => new_row.cells.push(Cell { value: v }),
                Err(e) => println!("Could not determine new value for byte {}: {}", e, byte),
            }
        }
        new_row
    }
}

impl Automaton for Rule110 {
    fn next(&mut self) {
        self.current_row = Rule110::apply_rule110(&self.current_row);
    }
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render(&self.current_row);
    }
}
