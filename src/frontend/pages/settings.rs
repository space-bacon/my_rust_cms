use yew::prelude::*;

#[function_component(Settings)]
pub fn settings() -> Html {
    html! {
        <div class="settings">
            <h2>{ "Settings" }</h2>
            { "Configure your site settings here." }
        </div>
    }
}
