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
            row.cells.push(Cell { value: rng.gen() });
        }
        Rule110 { current_row: row}
    }
    fn determine_new_value(byte: &u8) -> Result<bool, &'static str> {
        match byte {
            0b111 => Ok(false),
            0b110 => Ok(true),
            0b101 => Ok(true),
            0b100 => Ok(false),
            0b011 => Ok(true),
            0b010 => Ok(true),
            0b001 => Ok(true),
            0b000 => Ok(false),
            _ => Err("Unhandled case"),
        }
    }

    fn apply_rule110(row: &Row) -> Row {
        // TODO: Refactor
        let cols = row.len();
        let mut new_row = Row { cells: Vec::new() };
        let mut byte: u8 = 0b0000_0000;
        if row.cells[cols - 1].value {
            byte |= 0b0000_0100;
        }
        if row.cells[0].value {
            byte |= 0b0000_0010;
        }
        if row.cells[1].value {
            byte |= 0b0000_0001;
        }
        match Rule110::determine_new_value(&byte) {
            Ok(v) => new_row.cells.push(Cell { value: v }),
            Err(e) => panic!("Could not determine new value for byte {}: {}", e, byte),
        }
        for c in 1..=(cols - 2) {
            let mut byte: u8 = 0b0000_0000;
            if row.cells[c - 1].value {
                byte |= 0b0000_0100;
            }
            if row.cells[c].value {
                byte |= 0b0000_0010;
            }
            if row.cells[c + 1].value {
                byte |= 0b0000_0001;
            }
            let new_cell_value = Rule110::determine_new_value(&byte);
            match new_cell_value {
                Ok(v) => new_row.cells.push(Cell { value: v }),
                Err(e) => println!("Could not determine new value for byte {}: {}", e, byte),
            }
        }
        byte = 0b0000_0000;
        if row.cells[cols - 2].value {
            byte |= 0b0000_0100;
        }
        if row.cells[cols - 1].value {
            byte |= 0b0000_0010;
        }
        if row.cells[0].value {
            byte |= 0b0000_0001;
        }
        let new_cell_value = Rule110::determine_new_value(&byte);
        match new_cell_value {
            Ok(v) => new_row.cells.push(Cell { value: v }),
            Err(e) => panic!("Could not determine new value for byte {}: {}", e, byte),
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
