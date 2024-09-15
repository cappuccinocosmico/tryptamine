#![allow(non_snake_case)]

use crate::universe::{Cell, Universe};
use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

mod universe;

static CELL_SIZE: u32 = 5;
static GRID_COLOR: &str = "#CCCCCC";
static DEAD_COLOR: &str = "#FFFFFF";
static ALIVE_COLOR: &str = "#000000";

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let universe = Universe::new();
    let canvas_height = (CELL_SIZE + 1) * universe.height() + 1;
    let canvas_width = (CELL_SIZE + 1) * universe.width() + 1;

    use_effect!(cx, move |()| async move {
        let context = start_render_loop();
        loop {
            render_loop(context, universe);
            // Some async delay with https://docs.rs/gloo-timers/0.2.6/gloo_timers/, or https://docs.rs/async-std/latest/async_std/task/fn.sleep.html
            async_std::task::sleep(Duration::from_millis(10)).await;
        }
    ]);

    cx.render(rsx! {
        canvas { id: "game-of-life-canvas", height: canvas_height as i64, width: canvas_width as i64}
    })
}

fn start_render_loop(universe: Universe) -> web_sys::CanvasRenderingContext2d {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let canvas = document
        .get_element_by_id("game-of-life-canvas")
        .expect("expecting a canvas in the document")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context
}

fn render_loop(context: web_sys::CanvasRenderingContext2d, mut universe: Universe) {
    universe.tick();

    draw_cells(&context, &universe);
    draw_grid(&context, &universe);

    let closure = Closure::once(move || render_loop(context, universe));
    web_sys::window()
        .expect("global window does not exists")
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
}

fn draw_grid(context: &web_sys::CanvasRenderingContext2d, universe: &Universe) {
    context.begin_path();
    context.set_stroke_style(&GRID_COLOR.into());

    for i in 0..=universe.width() {
        context.move_to((i * (CELL_SIZE + 1) + 1) as f64, 0f64);
        context.line_to(
            (i * (CELL_SIZE + 1) + 1) as f64,
            ((CELL_SIZE + 1) * universe.height() + 1) as f64,
        );
    }

    for i in 0..=universe.height() {
        context.move_to(0f64, (i * (CELL_SIZE + 1) + 1) as f64);
        context.line_to(
            ((CELL_SIZE + 1) * universe.width() + 1) as f64,
            (i * (CELL_SIZE + 1) + 1) as f64,
        );
    }

    context.stroke();
}

fn draw_cells(context: &web_sys::CanvasRenderingContext2d, universe: &Universe) {
    context.begin_path();

    draw_cells_with_style(context, universe, |cell| cell == Cell::Alive, ALIVE_COLOR);
    draw_cells_with_style(context, universe, |cell| cell == Cell::Dead, DEAD_COLOR);

    context.stroke();
}

fn draw_cells_with_style(
    context: &web_sys::CanvasRenderingContext2d,
    universe: &Universe,
    condition: impl Fn(Cell) -> bool,
    style: &str,
) {
    context.set_fill_style(&style.into());
    for row in 0..universe.height() {
        for col in 0..universe.width() {
            let idx = universe[(row, col)];
            if !condition(universe.cells()[idx as usize]) {
                continue;
            }

            context.fill_rect(
                (col * (CELL_SIZE + 1) + 1) as f64,
                (row * (CELL_SIZE + 1) + 1) as f64,
                CELL_SIZE as f64,
                CELL_SIZE as f64,
            );
        }
    }
}
