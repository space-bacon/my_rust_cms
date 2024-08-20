use yew::prelude::*;

#[function_component(Preview)]
pub fn preview() -> Html {
    html! {
        <div class="preview">
            { "Live preview of the page" }
        </div>
    }
}
