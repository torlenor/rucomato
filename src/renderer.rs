use crate::row::Row;
use crate::grid::Grid;
use crate::events::Event;

pub trait Renderer {
    fn render(&mut self, row: &Row);
    fn render_grid(&mut self, grid: &Grid);
    fn get_events(&mut self) -> Vec<Event>;
    fn begin_render(&mut self) -> bool;
    fn end_render(&mut self);
}
