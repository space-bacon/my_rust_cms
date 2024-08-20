use yew::prelude::*;

#[function_component(PostEditor)]
pub fn post_editor() -> Html {
    html! {
        <div class="post-editor">
            <h2>{ "Edit Post" }</h2>
            <form>
                <input type="text" placeholder="Title" />
                <textarea placeholder="Content"></textarea>
                <button type="submit">{ "Save Post" }</button>
            </form>
        </div>
    }
}
