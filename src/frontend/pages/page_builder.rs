use yew::prelude::*;
use crate::components::builder::{Editor, Toolbox, Preview};

#[function_component(PageBuilder)]
pub fn page_builder() -> Html {
    html! {
        <div class="page-builder">
            <h2>{ "Page Builder" }</h2>
            <Toolbox />
            <Editor />
            <Preview />
        </div>
    }
}
