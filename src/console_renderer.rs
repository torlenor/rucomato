use crate::grid::Grid;
use crate::renderer::Renderer;
use crate::row::Row;
pub struct ConsoleRenderer {
    grid: bool,
}

impl ConsoleRenderer {
    pub fn new(grid: bool) -> ConsoleRenderer {
        ConsoleRenderer { grid }
    }
}

impl Renderer for ConsoleRenderer {
    fn render(&mut self, row: &Row) {
        println!("{}", row);
    }
    fn render_grid(&mut self, grid: &Grid) {
        print!("{}", grid);
    }
    fn begin_render(&mut self) -> bool {
        if self.grid {
            print!("\x1B[2J\x1B[1;1H");
        }
        false
    }
    fn end_render(&mut self) {}
}
