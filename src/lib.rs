pub mod car;
pub mod controls;
pub mod road;
pub mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/* #[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-self-driving-car!");
} */

/* #[wasm_bindgen]
pub fn infinite_loop() {
    log!("getting document");
    let document = web_sys::window()
        .expect("should have a window in this context")
        .document()
        .expect("window should have a document");

    log!("got document");

    log!("getting canvas");
    let canvas = document
        .get_element_by_id("myCanvas")
        .expect("canvas not found, have no target to paint to")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    log!("got canvas");

    canvas.set_width(200);

    log!("getting context");
    let ctx = canvas
        .get_context("2d")
        .expect("failed to get 2D context from canvas")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    log!("got context");
}

#[wasm_bindgen]
pub fn run() {
    use wasm_bindgen::JsCast;
    log!("hello from rust");

    let document = web_sys::window()
        .expect("should have a window in this context")
        .document()
        .expect("window should have a document");

    let canvas = document
        .get_element_by_id("myCanvas")
        .expect("canvas not found, have no target to paint to")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    canvas.set_width(200);

    let ctx = canvas
        .get_context("2d")
        .expect("failed to get 2D context from canvas")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let car = car::Car::new(100., 100., 30., 50.);
    /*
    loop {
        console_log!("loop running");
        car.draw(&ctx);
    } */
} */

#[wasm_bindgen]
pub fn test() {
    log!("testing testing testing");
}
