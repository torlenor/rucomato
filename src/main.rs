mod renderer;
use renderer::Renderer;

mod automaton;
use automaton::Automaton;

mod console_renderer;
use console_renderer::ConsoleRenderer;

mod sdl2_renderer;
use sdl2_renderer::Sdl2Renderer;

mod rule110;
use rule110::Rule110;
mod rule30;
use rule30::Rule30;
mod rule90;
use rule90::Rule90;
mod rule184;
use rule184::Rule184;
mod gol;
use gol::Gol;

mod cell;
mod grid;
mod row;

use std::{thread, time};

use std::env;

const CYCLE_TIME_MS: u64 = 100;

const COLS: usize = 100;
const ROWS: usize = 100;

const SDL_WINDOW_HEIGHT: u32 = 800;
const SDL_WINDOW_WIDTH: u32 = 800;

const SDL_CELL_HEIGHT: u32 = SDL_WINDOW_HEIGHT / (ROWS as u32);
const SDL_CELL_WIDTH: u32 = SDL_WINDOW_WIDTH / (COLS as u32);

pub fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("./rucomato automaton (rule110) output (console/sdl2)")
    }

    let automaton_select = &args[1];
    let output_select = &args[2];

    println!("Automaton: {}", automaton_select);
    println!("Output: {}", output_select);

    let mut renderer: Box<dyn Renderer>;

    let mut automaton: Box<dyn Automaton>;
    let mut grid = false;
    match automaton_select.as_str() {
        "rule110" => automaton = Box::new(Rule110::new(COLS)),
        "rule30" => automaton = Box::new(Rule30::new(COLS)),
        "rule90" => automaton = Box::new(Rule90::new(COLS)),
        "rule184" => automaton = Box::new(Rule184::new(COLS)),
        "gol" => {
            automaton = Box::new(Gol::new(ROWS, COLS));
            grid = true;
        }
        _ => panic!("Unknown automaton {} selected.", automaton_select),
    }

    match output_select.as_str() {
        "console" => renderer = Box::new(ConsoleRenderer::new(grid)),
        "sdl2" => {
            renderer = Box::new(Sdl2Renderer::new(
                automaton_select,
                SDL_WINDOW_WIDTH,
                SDL_WINDOW_HEIGHT,
                ROWS,
                SDL_CELL_WIDTH,
                SDL_CELL_HEIGHT,
            )?)
        }
        _ => panic!("Unknown output type {} selected.", output_select),
    }

    loop {
        let now = time::Instant::now();
        if renderer.begin_render() {
            break;
        }
        automaton.render(&mut *renderer);
        renderer.end_render();
        automaton.next();
        let elapsed = now.elapsed().as_millis() as u64;
        if output_select != "console" {
            println!("Iteration time: {} ms", elapsed);
        }
        if elapsed < CYCLE_TIME_MS {
            thread::sleep(time::Duration::from_millis(CYCLE_TIME_MS - elapsed));
        }
    }

    Ok(())
}
