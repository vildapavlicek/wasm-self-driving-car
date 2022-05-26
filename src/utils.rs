pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    console_error_panic_hook::set_once();
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}
