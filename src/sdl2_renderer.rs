extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::grid::Grid;
use crate::renderer::Renderer;
use crate::row::Row;
use crate::events;
use crate::cell::Cell;

#[allow(dead_code)]
const BLACK: sdl2::pixels::Color = Color::RGB(0, 0, 0);
const WHITE: sdl2::pixels::Color = Color::RGB(255, 255, 255);
const RED: sdl2::pixels::Color = Color::RGB(255, 0, 0);
const ORANGE: sdl2::pixels::Color = Color::RGB(253, 166, 47);
const BLUE: sdl2::pixels::Color = Color::RGB(0, 0, 255);

pub struct Sdl2Renderer {
    rows: usize,
    cell_width: u32,
    cell_height: u32,
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
    previous_rows: Vec<Row>,
}

impl Sdl2Renderer {
    pub fn new(
        title: &str,
        width: u32,
        height: u32,
        rows: usize,
        cell_width: u32,
        cell_height: u32,
    ) -> Result<Sdl2Renderer, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let event_pump = sdl_context.event_pump()?;

        Ok(Sdl2Renderer {
            rows: rows,
            cell_height: cell_height,
            cell_width: cell_width,
            canvas: canvas,
            event_pump: event_pump,
            previous_rows: Vec::new(),
        })
    }

    fn render_cell(&mut self, cell: Cell, col: usize, row: usize) {
        if cell.value == 1 {
            self.canvas.set_draw_color(BLUE);
            match self.canvas.fill_rect(sdl2::rect::Rect::new(
                (self.cell_width as i32) * (col as i32),
                (self.cell_height as i32) * (row as i32),
                self.cell_width,
                self.cell_height,
            )) {
                Err(e) => panic!("Error rendering rect: {}", e),
                _ => {}
            }
        } else if cell.value == 2 {
            self.canvas.set_draw_color(RED);
            match self.canvas.fill_rect(sdl2::rect::Rect::new(
                (self.cell_width as i32) * (col as i32),
                (self.cell_height as i32) * (row as i32),
                self.cell_width,
                self.cell_height,
            )) {
                Err(e) => panic!("Error rendering rect: {}", e),
                _ => {}
            }
        } else if cell.value == 3 {
            self.canvas.set_draw_color(ORANGE);
            match self.canvas.fill_rect(sdl2::rect::Rect::new(
                (self.cell_width as i32) * (col as i32),
                (self.cell_height as i32) * (row as i32),
                self.cell_width,
                self.cell_height,
            )) {
                Err(e) => panic!("Error rendering rect: {}", e),
                _ => {}
            }
        }
    } 
}

impl Renderer for Sdl2Renderer {
    fn render(&mut self, row: &Row) {
        self.previous_rows.push(row.clone());
        if self.previous_rows.len() >= self.rows {
            self.previous_rows.remove(0);
        }
        for row in 0..self.previous_rows.len() {
            for col in 0..self.previous_rows[row].cells.len() {
                self.render_cell(self.previous_rows[row].cells[col], col, row);
            }
        }
    }
    fn render_grid(&mut self, grid: &Grid) {
        for row in grid.rows.iter().enumerate() {
            for cell in row.1.cells.iter().enumerate() {
                self.render_cell(*cell.1, cell.0, row.0);
            }
        }
    }
    fn get_events(&mut self) -> Vec<events::Event>
    {
        let mut events : Vec<events::Event> = Vec::new();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => events.push(events::Event::QUIT),
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => events.push(events::Event::PAUSE),
                _ => {}
            }
        }
        events
    }
    fn begin_render(&mut self) -> bool {
        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();
        false
    }
    fn end_render(&mut self) {
        self.canvas.present();
    }
}
