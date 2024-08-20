use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1>{ "My Rust CMS" }</h1>
        </header>
    }
}
