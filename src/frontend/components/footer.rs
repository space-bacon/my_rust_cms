use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <p>{ "© 2024 My Rust CMS. All rights reserved." }</p>
        </footer>
    }
}
