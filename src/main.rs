#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;
#[cfg(target_arch = "wasm32")]
use yew::Renderer;

// WebAssembly entry point
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_app() {
    Renderer::<Model>::new().render();
}

// Yew component for Wasm
#[cfg(target_arch = "wasm32")]
struct Model;

#[cfg(target_arch = "wasm32")]
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <h1>{ "Hello, CMS!" }</h1>
        }
    }
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
