use std::fmt;

#[derive(Copy, Clone)]
pub struct Cell {
    pub value: i32,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value != 0 {
            write!(f, "■")
        } else {
            write!(f, "□")
        }
    }
}
