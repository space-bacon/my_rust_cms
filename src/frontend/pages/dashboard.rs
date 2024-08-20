use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="dashboard">
            <h2>{ "Dashboard" }</h2>
            { "Welcome to your dashboard." }
        </div>
    }
}
