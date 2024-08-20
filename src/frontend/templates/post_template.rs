use yew::prelude::*;

#[function_component(PostTemplate)]
pub fn post_template() -> Html {
    html! {
        <div class="post-template">
            <h1>{ "Post Title" }</h1>
            <div class="post-content">
                { "Post content goes here." }
            </div>
        </div>
    }
}
