use rand::Rng;

use crate::row::Row;
use crate::cell::Cell;

use crate::renderer::Renderer;
use crate::automaton::Automaton;

pub struct Rule30 {
    current_row: Row,
}

impl Rule30 {
    pub fn new(cols: usize) -> Rule30 {
        let mut row = Row { cells: vec![Cell{value: false}; cols] };
        row.cells[cols/2].value = true;
        Rule30 { current_row: row}
    }
    fn determine_new_value(byte: &u8) -> Result<bool, &'static str> {
        match byte {
            0b111 => Ok(false),
            0b110 => Ok(false),
            0b101 => Ok(false),
            0b100 => Ok(true),
            0b011 => Ok(true),
            0b010 => Ok(true),
            0b001 => Ok(true),
            0b000 => Ok(false),
            _ => Err("Unhandled case"),
        }
    }

    fn apply_rule30(row: &Row) -> Row {
        let cols = row.len();
        let mut new_row = Row { cells: Vec::new() };
        for c in 0..(cols) {
            let l_neib = ((c as i32)-1).rem_euclid(cols as i32) as usize;
            let r_neib = (c+1) % cols;
            let mut byte: u8 = 0b0000_0000;
            if row.cells[l_neib].value {
                byte |= 0b0000_0100;
            }
            if row.cells[c].value {
                byte |= 0b0000_0010;
            }
            if row.cells[r_neib].value {
                byte |= 0b0000_0001;
            }
            let new_cell_value = Rule30::determine_new_value(&byte);
            match new_cell_value {
                Ok(v) => new_row.cells.push(Cell { value: v }),
                Err(e) => println!("Could not determine new value for byte {}: {}", e, byte),
            }
        }
        new_row
    }
}

impl Automaton for Rule30 {
    fn next(&mut self) {
        self.current_row = Rule30::apply_rule30(&self.current_row);
    }
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.render(&self.current_row);
    }
}