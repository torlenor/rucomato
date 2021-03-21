use std::fmt;

use crate::row::Row;

#[derive(Clone)]
pub struct Grid {
    pub rows: Vec<Row>,
}

impl Grid {
    pub fn neib(&self, x: usize, y: usize, dir: u8) -> Result<(usize, usize), &'static str> {
        // Get the grid coordinates in the dir provided
        // 0 1 2
        // 3   4
        // 5 6 7
        // with periodic boundary conditions
        match dir {
            0 => {
                let xn = (x as i32 - 1).rem_euclid(self.rows[0].cells.len() as i32) as usize;
                let yn = (y as i32 - 1).rem_euclid(self.rows.len() as i32) as usize;
                Ok((xn, yn))
            }
            1 => {
                let yn = (y as i32 - 1).rem_euclid(self.rows.len() as i32) as usize;
                Ok((x, yn))
            }
            2 => {
                let xn = (x + 1) % self.rows[0].cells.len();
                let yn = (y as i32 - 1).rem_euclid(self.rows.len() as i32) as usize;
                Ok((xn, yn))
            }
            3 => {
                let xn = (x as i32 - 1).rem_euclid(self.rows[0].cells.len() as i32) as usize;
                Ok((xn, y))
            }
            4 => {
                let xn = (x + 1) % self.rows[0].cells.len();
                Ok((xn, y))
            }
            5 => {
                let xn = (x as i32 - 1).rem_euclid(self.rows[0].cells.len() as i32) as usize;
                let yn = (y + 1) % self.rows.len();
                Ok((xn, yn))
            }
            6 => {
                let yn = (y + 1) % self.rows.len();
                Ok((x, yn))
            }
            7 => {
                let xn = (x + 1) % self.rows[0].cells.len();
                let yn = (y + 1) % self.rows.len();
                Ok((xn, yn))
            }
            _ => Err("Wrong direction entered. Only valid from 0 to 7"),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid_str = String::new();
        for row in self.rows.iter().enumerate() {
            for cell in row.1.cells.iter() {
                grid_str.push_str(&cell.to_string());
            }
            if row.0 < self.rows.len() - 1 {
                grid_str.push_str("\n");
            }
        }
        write!(f, "{}", grid_str)
    }
}
