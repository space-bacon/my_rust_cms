use yew::prelude::*;

#[function_component(PostList)]
pub fn post_list() -> Html {
    html! {
        <div class="post-list">
            <h2>{ "Posts" }</h2>
            <ul>
                <li>{ "Post 1" }</li>
                <li>{ "Post 2" }</li>
                <li>{ "Post 3" }</li>
            </ul>
        </div>
    }
}
