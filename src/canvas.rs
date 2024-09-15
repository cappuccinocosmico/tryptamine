#![allow(non_snake_case)]

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
    let canvas_height = 500; // Height of the canvas
    let canvas_width = 500; // Width of the canvas

    use_effect!(cx, move |()| async move {
        let context = start_render_loop();
        loop {
            render_loop(context);
            async_std::task::sleep(Duration::from_millis(10)).await;
        }
    });

    cx.render(rsx! {
        canvas { id: "gradient-canvas", height: canvas_height as i64, width: canvas_width as i64}
    })
}

fn start_render_loop() -> web_sys::CanvasRenderingContext2d {
    let window = web_sys::window().expect("global window does not exist");
    let document = window.document().expect("expecting a document on window");
    let canvas = document
        .get_element_by_id("gradient-canvas")
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

fn render_loop(context: web_sys::CanvasRenderingContext2d) {
    draw_gradient(&context);

    let closure = Closure::once(move || render_loop(context));
    web_sys::window()
        .expect("global window does not exist")
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
}

fn draw_gradient(context: &web_sys::CanvasRenderingContext2d) {
    let canvas_width = context.canvas().unwrap().width() as f64;
    let canvas_height = context.canvas().unwrap().height() as f64;
    let gradient = context
        .create_linear_gradient(0.0, 0.0, canvas_width, canvas_height)
        .unwrap();

    gradient.add_color_stop(0.0, "white").unwrap();
    gradient.add_color_stop(1.0, "black").unwrap();

    context.set_fill_style(&gradient.into());
    context.fill_rect(0.0, 0.0, canvas_width, canvas_height);
}
