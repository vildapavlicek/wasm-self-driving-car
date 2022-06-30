use yew::prelude::*;
mod app;
use app::App;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        "starting wasm-self-driving-car",
    );

    yew::start_app::<App>();
}
