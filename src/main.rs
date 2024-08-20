#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

// Entry point for the WebAssembly version
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

// Basic Yew app component
#[cfg(target_arch = "wasm32")]
struct Model;

#[cfg(target_arch = "wasm32")]
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{ "Hello, CMS!" }</h1>
        }
    }
}

// Entry point for the native version
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Hello, CMS!");
}
