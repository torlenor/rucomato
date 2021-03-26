use std::fmt;

#[derive(Copy, Clone)]
pub struct Cell {
    pub value: i32,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value == 1 {
            write!(f, "■")
        } else if self.value == 2 {
            write!(f, "@")
        } else {
            write!(f, "□")
        }
    }
}
