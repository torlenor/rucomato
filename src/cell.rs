use std::fmt;

#[derive(Copy, Clone)]
pub struct Cell {
    pub value: bool,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value {
            write!(f, "■")
        } else {
            write!(f, "□")
        }
    }
}
