use yew::prelude::*;

#[function_component(PageTemplate)]
pub fn page_template() -> Html {
    html! {
        <div class="page-template">
            <h1>{ "Page Title" }</h1>
            <div class="page-content">
                { "Page content goes here." }
            </div>
        </div>
    }
}
