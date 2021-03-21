use crate::row::Row;
use crate::grid::Grid;

pub trait Renderer {
    fn render(&mut self, row: &Row);
    fn render_grid(&mut self, grid: &Grid);
    // TODO: event_poller
    fn begin_render(&mut self) -> bool;
    fn end_render(&mut self);
}
