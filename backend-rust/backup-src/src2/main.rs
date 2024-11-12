#![allow(non_snake_case)]

use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

static CELL_SIZE: u32 = 5;
static GRID_COLOR: &str = "#CCCCCC";
static DEAD_COLOR: &str = "#FFFFFF";
static ALIVE_COLOR: &str = "#000000";

fn main() {
    launch(App);
}

pub fn App() -> Element {
    rsx! (
                        p { class: "btn text-red-400",
                            "Dioxus is a new UI framework that makes it easy and simple to write cross-platform apps using web
                            technologies! It is functional, fast, and portable. Dioxus can run on the web, on the desktop, and
                            on mobile and embedded platforms."
                        }
    )
}

pub fn StacksIcon() -> Element {
    rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 24 24",
            path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" }
        }
    )
}

pub fn RightArrowIcon() -> Element {
    rsx!(
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-4 h-4 ml-1",
            view_box: "0 0 24 24",
            path { d: "M5 12h14M12 5l7 7-7 7" }
        }
    )
}

// fn App1(cx: Scope) -> Element {
//     let canvas_height = 500; // Height of the canvas
//     let canvas_width = 500; // Width of the canvas
//
//     use_effect!(cx, move |()| async move {
//         let context = start_render_loop();
//         loop {
//             render_loop(context);
//             async_std::task::sleep(Duration::from_millis(10)).await;
//         }
//     });
//
//     cx.render(rsx! {
//         canvas { id: "gradient-canvas", height: canvas_height as i64, width: canvas_width as i64}
//     })
// }
//
// fn start_render_loop() -> web_sys::CanvasRenderingContext2d {
//     let window = web_sys::window().expect("global window does not exist");
//     let document = window.document().expect("expecting a document on window");
//     let canvas = document
//         .get_element_by_id("gradient-canvas")
//         .expect("expecting a canvas in the document")
//         .dyn_into::<web_sys::HtmlCanvasElement>()
//         .unwrap();
//     let context = canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<web_sys::CanvasRenderingContext2d>()
//         .unwrap();
//
//     context
// }
//
// fn render_loop(context: web_sys::CanvasRenderingContext2d) {
//     draw_gradient(&context);
//
//     let closure = Closure::once(move || render_loop(context));
//     web_sys::window()
//         .expect("global window does not exist")
//         .request_animation_frame(closure.as_ref().unchecked_ref())
//         .unwrap();
// }
//
// fn draw_gradient(context: &web_sys::CanvasRenderingContext2d) {
//     let canvas_width = context.canvas().unwrap().width() as f64;
//     let canvas_height = context.canvas().unwrap().height() as f64;
//     let gradient = context
//         .create_linear_gradient(0.0, 0.0, canvas_width, canvas_height)
//         .unwrap();
//
//     gradient.add_color_stop(0.0, "white").unwrap();
//     gradient.add_color_stop(1.0, "black").unwrap();
//
//     context.set_fill_style(&gradient.into());
//     context.fill_rect(0.0, 0.0, canvas_width, canvas_height);
// }
