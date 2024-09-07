#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;
#[cfg(target_arch = "wasm32")]
use yew::Renderer;

mod frontend; // Make sure the frontend module is correctly included
use frontend::components::login_page::LoginPage; // Import your LoginPage component

// WebAssembly entry point
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_app() {
    Renderer::<LoginPage>::new().render(); // Render the LoginPage
}

// Native entry point for non-wasm32 builds
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Hello, native CMS!");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // WebAssembly has no main function
}
