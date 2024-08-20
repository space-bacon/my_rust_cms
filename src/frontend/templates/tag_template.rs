use yew::prelude::*;

#[function_component(TagTemplate)]
pub fn tag_template() -> Html {
    html! {
        <div class="tag-template">
            <h1>{ "Tag Name" }</h1>
            <ul>
                <li>{ "Post 1" }</li>
                <li>{ "Post 2" }</li>
                <li>{ "Post 3" }</li>
            </ul>
        </div>
    }
}
