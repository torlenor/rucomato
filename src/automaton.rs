use crate::renderer::Renderer;

pub trait Automaton {
    fn next(&mut self);
    fn render(&self, renderer: &mut dyn Renderer);
}
