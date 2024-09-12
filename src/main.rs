use wasm_bindgen::prelude::*;
use yew::Renderer;

mod frontend; // Make sure the frontend module is included
use frontend::components::login_page::LoginPage; // Import the LoginPage component

// WebAssembly entry point
#[wasm_bindgen(start)]
pub fn run_app() {
    Renderer::<LoginPage>::new().render(); // Render the LoginPage
}

fn main() {
    // Since this is a web-only application, WebAssembly has no native main function.
}
