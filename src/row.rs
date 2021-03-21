use std::fmt;

use crate::cell::Cell;

#[derive(Clone)]
pub struct Row {
    pub cells: Vec<Cell>,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut row = String::new();
        for cell in self.cells.iter() {
            row.push_str(&cell.to_string());
        }
        write!(f, "{}", row)
    }
}

impl Row {
    pub fn len(&self) -> usize {
        self.cells.len()
    }
}